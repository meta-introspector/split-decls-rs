// Generated macro for TakeRange (enum)
macro_rules! Depcrate_parser_rangeTakeRange {
() => {
// Module: crate::parser::range
// Provides: {"TakeRange"}
// Dependencies: {}
# [derive (Debug , PartialEq)] pub enum TakeRange { # [doc = " Found the pattern at this offset"] Found (usize) , # [doc = " Did not find the pattern but the parser can skip ahead to this offset."] NotFound (usize) , }
};
}
