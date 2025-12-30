// Generated macro for Error (struct)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " Parse-related error type."] # [derive (Clone , Debug , Eq , Hash , PartialEq)] pub struct Error < R > { # [doc = " Variant of the error"] pub variant : ErrorVariant < R > , # [doc = " Location within the input string"] pub location : InputLocation , # [doc = " Line/column within the input string"] pub line_col : LineColLocation , path : Option < String > , line : String , continued_line : Option < String > , parse_attempts : Option < ParseAttempts < R > > , }
};
}
