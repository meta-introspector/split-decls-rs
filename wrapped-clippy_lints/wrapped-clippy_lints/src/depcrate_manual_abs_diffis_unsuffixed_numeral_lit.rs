// Generated macro for is_unsuffixed_numeral_lit (function)
macro_rules! Depcrate_manual_abs_diffis_unsuffixed_numeral_lit {
() => {
// Module: crate::manual_abs_diff
// Provides: {"is_unsuffixed_numeral_lit"}
// Dependencies: {}
fn is_unsuffixed_numeral_lit (expr : & Expr < '_ >) -> bool { matches ! (expr . kind , ExprKind :: Lit (lit) if lit . node . is_numeric () && lit . node . is_unsuffixed ()) }
};
}
