// Generated macro for FormatterFormat (enum)
macro_rules! Depcrate_log_formatFormatterFormat {
() => {
// Module: crate::log::format
// Provides: {"FormatterFormat"}
// Dependencies: {}
# [derive (Debug)] # [non_exhaustive] pub enum FormatterFormat < 'a > { # [doc = " The classic defmt two-line format."] # [doc = ""] # [doc = " Looks like:"] # [doc = ""] # [doc = " ```text"] # [doc = " INFO This is a log message"] # [doc = " └─ test_lib::hello @ /Users/jonathan/Documents/knurling/test-lib/src/lib.rs:8"] # [doc = " ```"] Default { with_location : bool , } , # [doc = " A one-line format."] # [doc = ""] # [doc = " Looks like:"] # [doc = ""] # [doc = " ```text"] # [doc = " [INFO ] This is a log message (crate_name test-lib/src/lib.rs:8)"] # [doc = " ```"] OneLine { with_location : bool , } , Custom (& 'a str) , }
};
}
