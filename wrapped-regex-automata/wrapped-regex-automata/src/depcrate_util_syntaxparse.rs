// Generated macro for parse (function)
macro_rules! Depcrate_util_syntaxparse {
() => {
// Module: crate::util::syntax
// Provides: {"parse"}
// Dependencies: {}
# [doc = " A convenience routine for parsing a pattern into an HIR value with the"] # [doc = " default configuration."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " This shows how to parse a pattern into an HIR value:"] # [doc = ""] # [doc = " ```"] # [doc = " use regex_automata::util::syntax;"] # [doc = ""] # [doc = " let hir = syntax::parse(r\"([a-z]+)|([0-9]+)\")?;"] # [doc = " assert_eq!(Some(1), hir.properties().static_explicit_captures_len());"] # [doc = ""] # [doc = " # Ok::<(), Box<dyn std::error::Error>>(())"] # [doc = " ```"] pub fn parse (pattern : & str) -> Result < Hir , Error > { parse_with (pattern , & Config :: default ()) }
};
}
