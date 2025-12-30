// Generated macro for Error (struct)
macro_rules! Depcrate_parseError {
() => {
// Module: crate::parse
// Provides: {"Error"}
// Dependencies: {}
# [doc = " A parser error reports the one-indexed line number where the parsing error"] # [doc = " occurred, as well as the last parser node and the remaining data to be"] # [doc = " parsed."] # [derive (PartialEq , Debug)] pub struct Error { line_number : usize , last_attempted_parser : error :: ParseNode , parsed_until : bstr :: BString , }
};
}
