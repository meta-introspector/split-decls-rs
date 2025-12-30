// Generated macro for ValueSource (enum)
macro_rules! Depcrate_parser_matches_value_sourceValueSource {
() => {
// Module: crate::parser::matches::value_source
// Provides: {"ValueSource"}
// Dependencies: {}
# [doc = " Origin of the argument's value"] # [derive (Debug , Clone , Copy , PartialEq , Eq , PartialOrd , Ord)] # [non_exhaustive] pub enum ValueSource { # [doc = " Value came [`Arg::default_value`][crate::Arg::default_value]"] DefaultValue , # [doc = " Value came [`Arg::env`][crate::Arg::env]"] EnvVariable , # [doc = " Value was passed in on the command-line"] CommandLine , }
};
}
