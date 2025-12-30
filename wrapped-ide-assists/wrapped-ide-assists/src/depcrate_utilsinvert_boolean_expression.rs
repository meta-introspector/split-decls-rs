// Generated macro for invert_boolean_expression (function)
macro_rules! Depcrate_utilsinvert_boolean_expression {
() => {
// Module: crate::utils
// Provides: {"invert_boolean_expression"}
// Dependencies: {}
pub (crate) fn invert_boolean_expression (make : & SyntaxFactory , expr : ast :: Expr) -> ast :: Expr { invert_special_case (make , & expr) . unwrap_or_else (| | make . expr_prefix (T ! [!] , expr) . into ()) }
};
}
