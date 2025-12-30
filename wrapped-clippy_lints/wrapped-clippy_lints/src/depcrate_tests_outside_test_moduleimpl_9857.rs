// Generated macro for impl_9857 (impl)
macro_rules! Depcrate_tests_outside_test_moduleimpl_9857 {
() => {
// Module: crate::tests_outside_test_module
// Provides: {"impl_9857"}
// Dependencies: {}
impl LateLintPass < '_ > for TestsOutsideTestModule { fn check_fn (& mut self , cx : & LateContext < '_ > , kind : FnKind < '_ > , _ : & FnDecl < '_ > , body : & Body < '_ > , sp : Span , _ : LocalDefId ,) { if ! matches ! (kind , FnKind :: Closure) && is_in_test_function (cx . tcx , body . id () . hir_id) && ! is_in_cfg_test (cx . tcx , body . id () . hir_id) { # [expect (clippy :: collapsible_span_lint_calls , reason = "rust-clippy#7797")] span_lint_and_then (cx , TESTS_OUTSIDE_TEST_MODULE , sp , "this function marked with #[test] is outside a #[cfg(test)] module" , | diag | { diag . note ("move it to a testing module marked with #[cfg(test)]") ; } ,) ; } } }
};
}
