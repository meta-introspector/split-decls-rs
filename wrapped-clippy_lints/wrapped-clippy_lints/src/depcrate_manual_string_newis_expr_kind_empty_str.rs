// Generated macro for is_expr_kind_empty_str (function)
macro_rules! Depcrate_manual_string_newis_expr_kind_empty_str {
() => {
// Module: crate::manual_string_new
// Provides: {"is_expr_kind_empty_str"}
// Dependencies: {}
# [doc = " Checks if an expression's kind corresponds to an empty &str."] fn is_expr_kind_empty_str (expr_kind : & ExprKind < '_ >) -> bool { if let ExprKind :: Lit (lit) = expr_kind && let LitKind :: Str (value , _) = lit . node && value == sym :: empty { return true ; } false }
};
}
