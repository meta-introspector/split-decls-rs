// Generated macro for requires_semi (function)
macro_rules! Depcrate_macrequires_semi {
() => {
// Module: crate::mac
// Provides: {"requires_semi"}
// Dependencies: {}
pub (crate) fn requires_semi (delimiter : & MacroDelimiter) -> bool { match delimiter { MacroDelimiter :: Paren (_) | MacroDelimiter :: Bracket (_) => true , MacroDelimiter :: Brace (_) => false , } }
};
}
