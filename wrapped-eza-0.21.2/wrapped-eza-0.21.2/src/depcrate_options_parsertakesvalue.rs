// Generated macro for TakesValue (enum)
macro_rules! Depcrate_options_parserTakesValue {
() => {
// Module: crate::options::parser
// Provides: {"TakesValue"}
// Dependencies: {}
# [doc = " Whether a flag takes a value. This is applicable to both long and short"] # [doc = " arguments."] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum TakesValue { # [doc = " This flag has to be followed by a value."] # [doc = " If there’s a fixed set of possible values, they can be printed out"] # [doc = " with the error text."] Necessary (Option < Values >) , # [doc = " This flag will throw an error if there’s a value after it."] Forbidden , # [doc = " This flag may be followed by a value to override its defaults"] Optional (Option < Values > , & 'static str) , }
};
}
