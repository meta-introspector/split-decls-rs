// Generated macro for eq_pattern_length (function)
macro_rules! Depcrate_manual_stripeq_pattern_length {
() => {
// Module: crate::manual_strip
// Provides: {"eq_pattern_length"}
// Dependencies: {}
fn eq_pattern_length < 'tcx > (cx : & LateContext < 'tcx > , pattern : & Expr < '_ > , expr : & 'tcx Expr < '_ > , ctxt : SyntaxContext ,) -> bool { if let ExprKind :: Lit (Spanned { node : LitKind :: Int (n , _) , .. }) = expr . kind { constant_length (cx , pattern , ctxt) . is_some_and (| length | n == length) } else { len_arg (cx , expr) . is_some_and (| arg | eq_expr_value (cx , pattern , arg)) } }
};
}
