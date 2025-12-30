// Generated macro for requires_semi_to_be_stmt (function)
macro_rules! Depcrate_classifyrequires_semi_to_be_stmt {
() => {
// Module: crate::classify
// Provides: {"requires_semi_to_be_stmt"}
// Dependencies: {}
pub (crate) fn requires_semi_to_be_stmt (expr : & Expr) -> bool { match expr { Expr :: Macro (expr) => ! matches ! (expr . mac . delimiter , MacroDelimiter :: Brace (_)) , _ => requires_comma_to_be_match_arm (expr) , } }
};
}
