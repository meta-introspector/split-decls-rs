// Generated macro for Span (struct)
macro_rules! Depcrate_errorSpan {
() => {
// Module: crate::error
// Provides: {"Span"}
// Dependencies: {}
# [derive (Clone , Debug , PartialEq , Eq)] # [doc = " Spans select a range of text between two positions."] # [doc = " Spans are used in [`SpannedError`] to indicate the start and end positions"] # [doc = " of the parser cursor before and after it encountered an error in parsing."] pub struct Span { pub start : Position , pub end : Position , }
};
}
