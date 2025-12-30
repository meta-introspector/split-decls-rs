// Generated macro for Arg (struct)
macro_rules! Depcrate_options_parserArg {
() => {
// Module: crate::options::parser
// Provides: {"Arg"}
// Dependencies: {}
# [doc = " An **argument** can be matched by one of the user’s input strings."] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub struct Arg { # [doc = " The short argument that matches it, if any."] pub short : Option < ShortArg > , # [doc = " The long argument that matches it. This is non-optional; all flags"] # [doc = " should at least have a descriptive long name."] pub long : LongArg , # [doc = " Whether this flag takes a value or not."] pub takes_value : TakesValue , }
};
}
