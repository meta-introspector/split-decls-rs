// Generated macro for is_float_literal (function)
macro_rules! Depcrateis_float_literal {
() => {
// Module: crate
// Provides: {"is_float_literal"}
// Dependencies: {}
# [doc = " Checks whether the given expression is a constant literal of the given value."] pub fn is_float_literal (expr : & Expr < '_ > , value : f64) -> bool { if let ExprKind :: Lit (spanned) = expr . kind && let LitKind :: Float (v , _) = spanned . node { v . as_str () . parse () == Ok (value) } else { false } }
};
}
