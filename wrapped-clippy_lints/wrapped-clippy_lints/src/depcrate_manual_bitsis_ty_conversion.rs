// Generated macro for is_ty_conversion (function)
macro_rules! Depcrate_manual_bitsis_ty_conversion {
() => {
// Module: crate::manual_bits
// Provides: {"is_ty_conversion"}
// Dependencies: {}
fn is_ty_conversion (expr : & Expr < '_ >) -> bool { if let ExprKind :: Cast (..) = expr . kind { true } else if let ExprKind :: MethodCall (path , _ , [] , _) = expr . kind && path . ident . name == sym :: try_into { true } else { false } }
};
}
