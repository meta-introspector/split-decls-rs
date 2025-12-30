// Generated macro for match_acceptable_type (function)
macro_rules! Depcrate_manual_retainmatch_acceptable_type {
() => {
// Module: crate::manual_retain
// Provides: {"match_acceptable_type"}
// Dependencies: {}
fn match_acceptable_type (cx : & LateContext < '_ > , expr : & hir :: Expr < '_ > , msrv : Msrv) -> bool { let ty = cx . typeck_results () . expr_ty (expr) . peel_refs () ; let required = match ty . opt_diag_name (cx) { Some (sym :: BinaryHeap) => msrvs :: BINARY_HEAP_RETAIN , Some (sym :: BTreeSet) => msrvs :: BTREE_SET_RETAIN , Some (sym :: BTreeMap) => msrvs :: BTREE_MAP_RETAIN , Some (sym :: HashSet) => msrvs :: HASH_SET_RETAIN , Some (sym :: HashMap) => msrvs :: HASH_MAP_RETAIN , Some (sym :: Vec | sym :: VecDeque) => return true , _ => return false , } ; msrv . meets (cx , required) }
};
}
