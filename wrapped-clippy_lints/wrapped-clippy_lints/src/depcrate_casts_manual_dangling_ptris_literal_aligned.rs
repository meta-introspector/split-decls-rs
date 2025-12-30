// Generated macro for is_literal_aligned (function)
macro_rules! Depcrate_casts_manual_dangling_ptris_literal_aligned {
() => {
// Module: crate::casts::manual_dangling_ptr
// Provides: {"is_literal_aligned"}
// Dependencies: {}
fn is_literal_aligned (cx : & LateContext < '_ > , lit : & Spanned < LitKind > , to : & Ty < '_ >) -> bool { let LitKind :: Int (val , _) = lit . node else { return false } ; if val == 0 { return false ; } let to_mid_ty = cx . typeck_results () . node_type (to . hir_id) ; cx . tcx . layout_of (cx . typing_env () . as_query_input (to_mid_ty)) . is_ok_and (| layout | { let align = u128 :: from (layout . align . bytes ()) ; u128 :: from (val) <= align }) }
};
}
