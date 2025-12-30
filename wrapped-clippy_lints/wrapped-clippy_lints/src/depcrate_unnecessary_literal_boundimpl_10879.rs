// Generated macro for impl_10879 (impl)
macro_rules! Depcrate_unnecessary_literal_boundimpl_10879 {
() => {
// Module: crate::unnecessary_literal_bound
// Provides: {"impl_10879"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnnecessaryLiteralBound { fn check_fn (& mut self , cx : & LateContext < 'tcx > , kind : FnKind < 'tcx > , decl : & 'tcx FnDecl < '_ > , body : & 'tcx Body < '_ > , span : Span , _ : LocalDefId ,) { if span . from_expansion () { return ; } if matches ! (kind , FnKind :: Closure) { return ; } let FnRetTy :: Return (ret_hir_ty) = decl . output else { return ; } ; let Some (inner_hir_ty) = extract_anonymous_ref (ret_hir_ty) else { return ; } ; if ! matches ! (inner_hir_ty . basic_res () , Res :: PrimTy (PrimTy :: Str)) { return ; } if check_explicit_returns_static_str (body . value) && check_implicit_returns_static_str (body) { span_lint_and_sugg (cx , UNNECESSARY_LITERAL_BOUND , ret_hir_ty . span , "returning a `str` unnecessarily tied to the lifetime of arguments" , "try" , "&'static str" . into () , Applicability :: MachineApplicable ,) ; } } }
};
}
