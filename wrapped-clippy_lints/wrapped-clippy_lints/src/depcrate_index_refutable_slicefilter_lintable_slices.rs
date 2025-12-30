// Generated macro for filter_lintable_slices (function)
macro_rules! Depcrate_index_refutable_slicefilter_lintable_slices {
() => {
// Module: crate::index_refutable_slice
// Provides: {"filter_lintable_slices"}
// Dependencies: {}
fn filter_lintable_slices < 'tcx > (cx : & LateContext < 'tcx > , slice_lint_info : FxIndexMap < HirId , SliceLintInformation > , max_suggested_slice : u64 , scope : & 'tcx hir :: Expr < 'tcx > ,) -> FxIndexMap < HirId , SliceLintInformation > { let mut visitor = SliceIndexLintingVisitor { cx , slice_lint_info , max_suggested_slice , } ; intravisit :: walk_expr (& mut visitor , scope) ; visitor . slice_lint_info }
};
}
