// Generated macro for impl_1225 (impl)
macro_rules! Depcrate_collection_is_never_readimpl_1225 {
() => {
// Module: crate::collection_is_never_read
// Provides: {"impl_1225"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for CollectionIsNeverRead { fn check_local (& mut self , cx : & LateContext < 'tcx > , local : & 'tcx LetStmt < 'tcx >) { if let PatKind :: Binding (_ , local_id , _ , _) = local . pat . kind && match_acceptable_type (cx , local) && let Some (enclosing_block) = get_enclosing_block (cx , local . hir_id) && has_no_read_access (cx , local_id , enclosing_block) { span_lint (cx , COLLECTION_IS_NEVER_READ , local . span , "collection is never read") ; } } }
};
}
