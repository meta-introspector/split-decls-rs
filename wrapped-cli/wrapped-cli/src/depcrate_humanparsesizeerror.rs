// Generated macro for ParseSizeError (struct)
macro_rules! Depcrate_humanParseSizeError {
() => {
// Module: crate::human
// Provides: {"ParseSizeError"}
// Dependencies: {}
# [doc = " An error that occurs when parsing a human readable size description."] # [doc = ""] # [doc = " This error provides an end user friendly message describing why the"] # [doc = " description couldn't be parsed and what the expected format is."] # [derive (Clone , Debug , Eq , PartialEq)] pub struct ParseSizeError { original : String , kind : ParseSizeErrorKind , }
};
}
