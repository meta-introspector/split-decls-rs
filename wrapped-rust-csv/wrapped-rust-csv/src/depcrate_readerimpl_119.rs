// Generated macro for impl_119 (impl)
macro_rules! Depcrate_readerimpl_119 {
() => {
// Module: crate::reader
// Provides: {"impl_119"}
// Dependencies: {}
impl Reader < Reader < File > > { # [doc = " Create a new CSV parser with a default configuration for the given"] # [doc = " file path."] # [doc = ""] # [doc = " To customize CSV parsing, use a `ReaderBuilder`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::error::Error;"] # [doc = " use csv::Reader;"] # [doc = ""] # [doc = " # fn main() { example().unwrap(); }"] # [doc = " fn example() -> Result<(), Box<dyn Error>> {"] # [doc = "     let mut rdr = Reader::from_path(\"foo.csv\")?;"] # [doc = "     for result in rdr.records() {"] # [doc = "         let record = result?;"] # [doc = "         println!(\"{:?}\", record);"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] pub fn from_path < P : AsRef < Path > > (path : P) -> Result < Reader < File > > { ReaderBuilder :: new () . from_path (path) } }
};
}
