// Generated macro for Error (enum)
macro_rules! Depcrate_errorError {
() => {
// Module: crate::error
// Provides: {"Error"}
// Dependencies: {}
# [doc = " All possible errors which can occur when calling one of the public macros."] # [derive (Debug , PartialEq , Clone)] pub enum Error { # [doc = " Error during the initial parsing of the macro arguments."] Parse (String) , # [doc = " The first macro argument is not a string literal."] MustBeStringLiteral , # [doc = " Unable to parse a tag."] UnableToParseTag (String) , # [doc = " An error occured while parsing a color tag."] ParseTag (String) , # [doc = " A \"{\" character has not been closed in the format string."] UnclosedPlaceholder , # [doc = " A \"<\" character has not been closed in the format string."] UnclosedTag , # [doc = " Trying to close a previous tag, while there are no open tag."] NoTagToClose , # [doc = " Trying to close a previous tag which does not match, like `<red>...</blue>`."] MismatchCloseTag (String , String) , # [doc = " Only one argument is allowed for the `cstr!()` and `untagged!()` macros."] # [cfg (not (feature = "terminfo"))] TooManyArgs , # [doc = " Only one argument is allowed for the '`untagged!()` macro."] # [cfg (feature = "terminfo")] TooManyArgs , }
};
}
