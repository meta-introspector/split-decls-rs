// Generated macro for is_integer_literal (function)
macro_rules! Depcrateis_integer_literal {
() => {
// Module: crate
// Provides: {"is_integer_literal"}
// Dependencies: {}
# [doc = " Checks whether the given expression is a constant literal of the given value."] pub fn is_integer_literal (expr : & Expr < '_ > , value : u128) -> bool { if let ExprKind :: Lit (spanned) = expr . kind && let LitKind :: Int (v , _) = spanned . node { return v == value ; } false }
};
}
