// Generated macro for impl_7526 (impl)
macro_rules! Depcrate_multiple_unsafe_ops_per_blockimpl_7526 {
() => {
// Module: crate::multiple_unsafe_ops_per_block
// Provides: {"impl_7526"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MultipleUnsafeOpsPerBlock { fn check_block (& mut self , cx : & LateContext < 'tcx > , block : & 'tcx hir :: Block < '_ >) { if ! matches ! (block . rules , BlockCheckMode :: UnsafeBlock (_)) || block . span . in_external_macro (cx . tcx . sess . source_map ()) || block . span . is_desugaring (DesugaringKind :: Await) { return ; } let mut unsafe_ops = vec ! [] ; collect_unsafe_exprs (cx , block , & mut unsafe_ops) ; if unsafe_ops . len () > 1 { span_lint_and_then (cx , MULTIPLE_UNSAFE_OPS_PER_BLOCK , block . span , format ! ("this `unsafe` block contains {} unsafe operations, expected only one" , unsafe_ops . len ()) , | diag | { for (msg , span) in unsafe_ops { diag . span_note (span , msg) ; } } ,) ; } } }
};
}
