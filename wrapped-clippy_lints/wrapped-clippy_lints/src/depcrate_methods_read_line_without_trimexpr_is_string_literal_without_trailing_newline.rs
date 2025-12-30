// Generated macro for expr_is_string_literal_without_trailing_newline (function)
macro_rules! Depcrate_methods_read_line_without_trimexpr_is_string_literal_without_trailing_newline {
() => {
// Module: crate::methods::read_line_without_trim
// Provides: {"expr_is_string_literal_without_trailing_newline"}
// Dependencies: {}
fn expr_is_string_literal_without_trailing_newline (expr : & Expr < '_ >) -> bool { if let ExprKind :: Lit (lit) = expr . kind && let LitKind :: Str (sym , _) = lit . node { ! sym . as_str () . ends_with ('\n') } else { false } }
};
}
