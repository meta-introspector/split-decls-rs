// Generated macro for impl_9644 (impl)
macro_rules! Depcrate_returnsimpl_9644 {
() => {
// Module: crate::returns
// Provides: {"impl_9644"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for Return { fn check_stmt (& mut self , cx : & LateContext < 'tcx > , stmt : & 'tcx Stmt < '_ >) { needless_return_with_question_mark :: check_stmt (cx , stmt) ; } fn check_block (& mut self , cx : & LateContext < 'tcx > , block : & 'tcx Block < '_ >) { let_and_return :: check_block (cx , block) ; } fn check_fn (& mut self , cx : & LateContext < 'tcx > , kind : FnKind < 'tcx > , _ : & 'tcx FnDecl < 'tcx > , body : & 'tcx Body < 'tcx > , sp : Span , _ : LocalDefId ,) { needless_return :: check_fn (cx , kind , body , sp) ; } }
};
}
