// Generated macro for ParseError (enum)
macro_rules! Depcrate_options_parserParseError {
() => {
// Module: crate::options::parser
// Provides: {"ParseError"}
// Dependencies: {}
# [doc = " A problem with the user’s input that meant it couldn’t be parsed into a"] # [doc = " coherent list of arguments."] # [derive (PartialEq , Eq , Debug)] pub enum ParseError { # [doc = " A flag that has to take a value was not given one."] NeedsValue { flag : Flag , values : Option < Values > } , # [doc = " A flag that can’t take a value *was* given one."] ForbiddenValue { flag : Flag } , # [doc = " A short argument, either alone or in a cluster, was not"] # [doc = " recognised by the program."] UnknownShortArgument { attempt : ShortArg } , # [doc = " A long argument was not recognised by the program."] # [doc = " We don’t have a known &str version of the flag, so"] # [doc = " this may not be valid UTF-8."] UnknownArgument { attempt : OsString } , }
};
}
