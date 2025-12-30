// Generated macro for OptionsError (enum)
macro_rules! Depcrate_options_errorOptionsError {
() => {
// Module: crate::options::error
// Provides: {"OptionsError"}
// Dependencies: {}
# [doc = " Something wrong with the combination of options the user has picked."] # [derive (PartialEq , Eq , Debug)] pub enum OptionsError { # [doc = " There was an error (from `getopts`) parsing the arguments."] Parse (ParseError) , # [doc = " The user supplied an illegal choice to an Argument."] BadArgument (& 'static Arg , OsString) , # [doc = " The user supplied a set of options that are unsupported"] Unsupported (String) , # [doc = " An option was given twice or more in strict mode."] Duplicate (Flag , Flag) , # [doc = " Two options were given that conflict with one another."] Conflict (& 'static Arg , & 'static Arg) , # [doc = " An option was given that does nothing when another one either is or"] # [doc = " isn’t present."] Useless (& 'static Arg , bool , & 'static Arg) , # [doc = " An option was given that does nothing when either of two other options"] # [doc = " are not present."] Useless2 (& 'static Arg , & 'static Arg , & 'static Arg) , # [doc = " A very specific edge case where --tree can’t be used with --all twice."] TreeAllAll , # [doc = " A numeric option was given that failed to be parsed as a number."] FailedParse (String , NumberSource , ParseIntError) , # [doc = " A glob ignore was given that failed to be parsed as a pattern."] FailedGlobPattern (String) , }
};
}
