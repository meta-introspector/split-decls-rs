// Generated macro for extract_bool_lit (function)
macro_rules! Depcrate_bool_assert_comparisonextract_bool_lit {
() => {
// Module: crate::bool_assert_comparison
// Provides: {"extract_bool_lit"}
// Dependencies: {}
fn extract_bool_lit (e : & Expr < '_ >) -> Option < bool > { if let ExprKind :: Lit (Lit { node : LitKind :: Bool (b) , .. }) = e . kind && ! e . span . from_expansion () { Some (b) } else { None } }
};
}
