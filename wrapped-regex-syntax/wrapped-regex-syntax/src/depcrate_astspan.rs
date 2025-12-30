// Generated macro for Span (struct)
macro_rules! Depcrate_astSpan {
() => {
// Module: crate::ast
// Provides: {"Span"}
// Dependencies: {}
# [doc = " Span represents the position information of a single AST item."] # [doc = ""] # [doc = " All span positions are absolute byte offsets that can be used on the"] # [doc = " original regular expression that was parsed."] # [derive (Clone , Copy , Eq , PartialEq)] # [cfg_attr (feature = "arbitrary" , derive (arbitrary :: Arbitrary))] pub struct Span { # [doc = " The start byte offset."] pub start : Position , # [doc = " The end byte offset."] pub end : Position , }
};
}
