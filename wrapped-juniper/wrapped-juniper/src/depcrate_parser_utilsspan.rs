// Generated macro for Span (struct)
macro_rules! Depcrate_parser_utilsSpan {
() => {
// Module: crate::parser::utils
// Provides: {"Span"}
// Dependencies: {}
# [doc = " Range of characters in the input source, starting at the character pointed by the `start` field"] # [doc = " and ending just before the `end` marker."] # [derive (Clone , Copy , Debug , Eq , Hash , PartialEq)] pub struct Span { # [doc = " Start position of this [`Span`]."] pub start : SourcePosition , # [doc = " End position of this [`Span`]."] # [doc = ""] # [doc = " > __NOTE__: This points to the first source position __after__ this [`Span`]."] pub end : SourcePosition , }
};
}
