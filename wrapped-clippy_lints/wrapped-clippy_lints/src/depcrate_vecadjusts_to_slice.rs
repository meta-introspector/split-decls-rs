// Generated macro for adjusts_to_slice (function)
macro_rules! Depcrate_vecadjusts_to_slice {
() => {
// Module: crate::vec
// Provides: {"adjusts_to_slice"}
// Dependencies: {}
fn adjusts_to_slice (cx : & LateContext < '_ > , e : & Expr < '_ >) -> bool { matches ! (cx . typeck_results () . expr_ty_adjusted (e) . kind () , ty :: Ref (_ , ty , _) if ty . is_slice ()) }
};
}
