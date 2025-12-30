// Generated macro for impl_9215 (impl)
macro_rules! Depcrate_rc_clone_in_vec_initimpl_9215 {
() => {
// Module: crate::rc_clone_in_vec_init
// Provides: {"impl_9215"}
// Dependencies: {}
impl LateLintPass < '_ > for RcCloneInVecInit { fn check_expr (& mut self , cx : & LateContext < '_ > , expr : & Expr < '_ >) { let Some (macro_call) = root_macro_call_first_node (cx , expr) else { return ; } ; let Some (VecArgs :: Repeat (elem , len)) = VecArgs :: hir (cx , expr) else { return ; } ; let Some ((symbol , func_span)) = ref_init (cx , elem) else { return ; } ; emit_lint (cx , symbol , macro_call . span , elem , len , func_span) ; } }
};
}
