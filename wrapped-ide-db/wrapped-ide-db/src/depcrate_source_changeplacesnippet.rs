// Generated macro for PlaceSnippet (enum)
macro_rules! Depcrate_source_changePlaceSnippet {
() => {
// Module: crate::source_change
// Provides: {"PlaceSnippet"}
// Dependencies: {}
enum PlaceSnippet { # [doc = " Place a tabstop before an element"] Before (SyntaxElement) , # [doc = " Place a tabstop before an element"] After (SyntaxElement) , # [doc = " Place a placeholder snippet in place of the element"] Over (SyntaxElement) , # [doc = " Place a group of placeholder snippets which are linked together"] # [doc = " in place of the elements"] OverGroup (Vec < SyntaxElement >) , }
};
}
