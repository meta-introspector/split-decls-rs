// Generated macro for impl_7359 (impl)
macro_rules! Depcrate_missing_const_for_thread_localimpl_7359 {
() => {
// Module: crate::missing_const_for_thread_local
// Provides: {"impl_7359"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for MissingConstForThreadLocal { fn check_fn (& mut self , cx : & LateContext < 'tcx > , fn_kind : intravisit :: FnKind < 'tcx > , _ : & 'tcx rustc_hir :: FnDecl < 'tcx > , body : & 'tcx rustc_hir :: Body < 'tcx > , span : rustc_span :: Span , local_defid : rustc_span :: def_id :: LocalDefId ,) { let defid = local_defid . to_def_id () ; if is_thread_local_initializer (cx , fn_kind , span) . unwrap_or (false) && ! cx . tcx . is_const_fn (defid) && let ExprKind :: Block (block , _) = body . value . kind && let Some (unpeeled) = block . expr && let ret_expr = peel_blocks (unpeeled) && ! is_unreachable (cx , ret_expr) && initializer_can_be_made_const (cx , defid , self . msrv) && let initializer_snippet = snippet (cx , ret_expr . span , "thread_local! { ... }") && initializer_snippet != "thread_local! { ... }" && self . msrv . meets (cx , msrvs :: THREAD_LOCAL_CONST_INIT) { span_lint_and_sugg (cx , MISSING_CONST_FOR_THREAD_LOCAL , unpeeled . span , "initializer for `thread_local` value can be made `const`" , "replace with" , format ! ("const {{ {initializer_snippet} }}") , Applicability :: MachineApplicable ,) ; } } }
};
}
