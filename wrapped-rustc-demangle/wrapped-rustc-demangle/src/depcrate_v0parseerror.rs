// Generated macro for ParseError (enum)
macro_rules! Depcrate_v0ParseError {
() => {
// Module: crate::v0
// Provides: {"ParseError"}
// Dependencies: {}
# [derive (PartialEq , Eq , Debug)] pub enum ParseError { # [doc = " Symbol doesn't match the expected `v0` grammar."] Invalid , # [doc = " Parsing the symbol crossed the recursion limit (see `MAX_DEPTH`)."] RecursedTooDeep , }
};
}
