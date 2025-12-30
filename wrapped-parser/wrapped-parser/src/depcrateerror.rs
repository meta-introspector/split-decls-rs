// Generated macro for Error (enum)
macro_rules! DepcrateError {
() => {
// Module: crate
// Provides: {"Error"}
// Dependencies: {}
# [doc = " The kinds of error this library can return"] # [derive (thiserror :: Error , Debug , PartialEq , Eq , Clone)] pub enum Error { # [error ("invalid type specifier `{0:?}`")] InvalidTypeSpecifier (String) , # [error ("unable to parse given integer")] InvalidInteger (# [from] std :: num :: ParseIntError) , # [error ("invalid array specifier (missing length)")] InvalidArraySpecifierMissingLength , # [error ("invalid array specifier (missing `]`")] InvalidArraySpecifierMissingBracket , # [error ("trailing data after bitfield range")] TrailingDataAfterBitfieldRange , # [error ("malformed format string (missing display hint after ':')")] MalformedFormatString , # [error ("unknown display hint: {0:?}")] UnknownDisplayHint (String) , # [error ("unexpected content `{0:?}` in format string")] UnexpectedContentInFormatString (String) , # [error ("unmatched `{{` in format string")] UnmatchedOpenBracket , # [error ("unmatched `}}` in format string")] UnmatchedCloseBracket , # [error ("conflicting types for argument {0}: used as {1:?} and {2:?}")] ConflictingTypes (usize , Type , Type) , # [error ("argument {0} is not used in this format string")] UnusedArgument (usize) , }
};
}
