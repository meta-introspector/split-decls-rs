// Generated macro for impl_2854 (impl)
macro_rules! Depcrate_index_refutable_sliceimpl_2854 {
() => {
// Module: crate::index_refutable_slice
// Provides: {"impl_2854"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for IndexRefutableSlice { fn check_expr (& mut self , cx : & LateContext < 'tcx > , expr : & 'tcx hir :: Expr < '_ >) { if let Some (IfLet { let_pat , if_then , .. }) = IfLet :: hir (cx , expr) && ! expr . span . from_expansion () && ! is_lint_allowed (cx , INDEX_REFUTABLE_SLICE , expr . hir_id) && let found_slices = find_slice_values (cx , let_pat) && ! found_slices . is_empty () && let filtered_slices = filter_lintable_slices (cx , found_slices , self . max_suggested_slice , if_then) && ! filtered_slices . is_empty () && self . msrv . meets (cx , msrvs :: SLICE_PATTERNS) { for slice in filtered_slices . values () { lint_slice (cx , slice) ; } } } }
};
}
