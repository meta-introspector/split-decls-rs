// Generated macro for invert_boolean_expression_legacy (function)
macro_rules! Depcrate_utilsinvert_boolean_expression_legacy {
() => {
// Module: crate::utils
// Provides: {"invert_boolean_expression_legacy"}
// Dependencies: {}
pub (crate) fn invert_boolean_expression_legacy (expr : ast :: Expr) -> ast :: Expr { invert_special_case_legacy (& expr) . unwrap_or_else (| | make :: expr_prefix (T ! [!] , expr) . into ()) }
};
}
