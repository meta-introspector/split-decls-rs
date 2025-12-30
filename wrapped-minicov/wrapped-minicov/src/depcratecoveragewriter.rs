// Generated macro for CoverageWriter (trait)
macro_rules! DepcrateCoverageWriter {
() => {
// Module: crate
// Provides: {"CoverageWriter"}
// Dependencies: {}
# [doc = " Sink into which coverage data can be written."] # [doc = ""] # [doc = " A default implementation for `Vec<u8>` is provided,"] pub trait CoverageWriter { # [doc = " Writes the given bytes to the sink."] # [doc = ""] # [doc = " This method should return an error if all bytes could not be written to"] # [doc = " the sink."] fn write (& mut self , data : & [u8]) -> Result < () , CoverageWriteError > ; }
};
}
