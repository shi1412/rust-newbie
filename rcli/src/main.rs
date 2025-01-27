// rcli csv -i input.csv -o oiutput.json --heaeder -d ','
use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about=None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand
}
#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Conver CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long)]
    input: String,

    #[arg(short, long, default_value = "output.json")]
    output: String,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(long, default_value_t = true)]
    header: bool

}

fn main() {
    let opts: Opts = Opts::parse();
    println!("{:?}", opts);
}
