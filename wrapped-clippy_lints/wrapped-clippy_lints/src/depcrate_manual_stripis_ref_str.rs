// Generated macro for is_ref_str (function)
macro_rules! Depcrate_manual_stripis_ref_str {
() => {
// Module: crate::manual_strip
// Provides: {"is_ref_str"}
// Dependencies: {}
fn is_ref_str (cx : & LateContext < '_ > , expr : & Expr < '_ >) -> bool { match cx . typeck_results () . expr_ty_adjusted (expr) . kind () { ty :: Ref (_ , ty , _) => ty . is_str () , _ => false , } }
};
}
