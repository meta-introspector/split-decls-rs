// Generated macro for impl_2400 (impl)
macro_rules! Depcrate_from_str_radix_10impl_2400 {
() => {
// Module: crate::from_str_radix_10
// Provides: {"impl_2400"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for FromStrRadix10 { fn check_expr (& mut self , cx : & LateContext < 'tcx > , exp : & Expr < 'tcx >) { if let ExprKind :: Call (maybe_path , [src , radix]) = & exp . kind && let ExprKind :: Path (QPath :: TypeRelative (ty , pathseg)) = & maybe_path . kind && is_integer_literal (radix , 10) && pathseg . ident . name == sym :: from_str_radix && let TyKind :: Path (ty_qpath) = & ty . kind && let ty_res = cx . qpath_res (ty_qpath , ty . hir_id) && let def :: Res :: PrimTy (prim_ty) = ty_res && matches ! (prim_ty , PrimTy :: Int (_) | PrimTy :: Uint (_)) && ! is_in_const_context (cx) { let expr = if let ExprKind :: AddrOf (_ , _ , expr) = & src . kind { let ty = cx . typeck_results () . expr_ty (expr) ; if is_ty_stringish (cx , ty) { expr } else { & src } } else { & src } ; let sugg = Sugg :: hir_with_applicability (cx , expr , "<string>" , & mut Applicability :: MachineApplicable) . maybe_paren () ; span_lint_and_sugg (cx , FROM_STR_RADIX_10 , exp . span , "this call to `from_str_radix` can be replaced with a call to `str::parse`" , "try" , format ! ("{sugg}.parse::<{}>()" , prim_ty . name_str ()) , Applicability :: MaybeIncorrect ,) ; } } }
};
}
