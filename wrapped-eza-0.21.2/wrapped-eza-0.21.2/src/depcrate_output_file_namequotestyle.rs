// Generated macro for QuoteStyle (enum)
macro_rules! Depcrate_output_file_nameQuoteStyle {
() => {
// Module: crate::output::file_name
// Provides: {"QuoteStyle"}
// Dependencies: {}
# [doc = " Whether or not to wrap file names with spaces in quotes."] # [derive (PartialEq , Debug , Copy , Clone)] pub enum QuoteStyle { # [doc = " Don't ever quote file names."] NoQuotes , # [doc = " Use single quotes for file names that contain spaces and no single quotes"] # [doc = " Use double quotes for file names that contain single quotes."] QuoteSpaces , }
};
}
