// Generated macro for impl_7666 (impl)
macro_rules! Depcrate_multiple_unsafe_ops_per_blockimpl_7666 {
() => {
// Module: crate::multiple_unsafe_ops_per_block
// Provides: {"impl_7666"}
// Dependencies: {}
impl < 'tcx > UnsafeExprCollector < 'tcx > { fn collect_unsafe_exprs (cx : & LateContext < 'tcx > , block : & 'tcx hir :: Block < 'tcx >) -> Vec < (& 'static str , Span) > { let mut collector = Self { tcx : cx . tcx , typeck_results : cx . typeck_results () , unsafe_ops : vec ! [] , } ; collector . visit_block (block) ; collector . unsafe_ops } }
};
}
