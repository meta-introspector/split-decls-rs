// Generated macro for impl_208 (impl)
macro_rules! Depcrate_writerimpl_208 {
() => {
// Module: crate::writer
// Provides: {"impl_208"}
// Dependencies: {}
impl Writer < File > { # [doc = " Build a CSV writer with a default configuration that writes data to the"] # [doc = " given file path. The file is truncated if it already exists."] # [doc = ""] # [doc = " If there was a problem opening the file at the given path, then this"] # [doc = " returns the corresponding error."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::error::Error;"] # [doc = " use csv::Writer;"] # [doc = ""] # [doc = " # fn main() { example().unwrap(); }"] # [doc = " fn example() -> Result<(), Box<dyn Error>> {"] # [doc = "     let mut wtr = Writer::from_path(\"foo.csv\")?;"] # [doc = "     wtr.write_record(&[\"a\", \"b\", \"c\"])?;"] # [doc = "     wtr.write_record(&[\"x\", \"y\", \"z\"])?;"] # [doc = "     wtr.flush()?;"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] pub fn from_path < P : AsRef < Path > > (path : P) -> Result < Writer < File > > { WriterBuilder :: new () . from_path (path) } }
};
}
