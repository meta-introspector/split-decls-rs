// Generated macro for parser_never_panics (function)
macro_rules! Depcrate_propertiesparser_never_panics {
() => {
// Module: crate::properties
// Provides: {"parser_never_panics"}
// Dependencies: {}
# [test] fn parser_never_panics () { fn prop (s : RegexLikeString) -> bool { let _ = Expr :: parse (& s . 0) ; true } qc (prop as fn (RegexLikeString) -> bool) ; }
};
}
