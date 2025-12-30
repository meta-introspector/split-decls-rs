// Generated macro for impl_1164 (impl)
macro_rules! Depcrate_coerce_container_to_anyimpl_1164 {
() => {
// Module: crate::coerce_container_to_any
// Provides: {"impl_1164"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for CoerceContainerToAny { fn check_expr (& mut self , cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ >) { if ! cx . typeck_results () . expr_adjustments (e) . last () . is_some_and (| adj | { matches ! (adj . kind , Adjust :: Pointer (PointerCoercion :: Unsize)) && is_ref_dyn_any (cx . tcx , adj . target) }) { return ; } let expr_ty = cx . typeck_results () . expr_ty (e) ; let ty :: Ref (_ , expr_ref_ty , _) = * expr_ty . kind () else { return ; } ; if is_dyn_any (cx . tcx , expr_ref_ty) { return ; } let Some ((depth , target)) = clippy_utils :: ty :: deref_chain (cx , expr_ref_ty) . enumerate () . last () else { return ; } ; if ! is_dyn_any (cx . tcx , target) { return ; } let (target_expr , deref_count) = match e . kind { ExprKind :: AddrOf (_ , _ , referent) => (referent , depth) , _ => (e , depth + 1) , } ; let ty :: Ref (_ , _ , mutability) = * cx . typeck_results () . expr_ty_adjusted (e) . kind () else { return ; } ; let sugg = sugg :: make_unop (& format ! ("{}{}" , mutability . ref_prefix_str () , str :: repeat ("*" , deref_count)) , Sugg :: hir (cx , target_expr , "..") ,) ; span_lint_and_sugg (cx , COERCE_CONTAINER_TO_ANY , e . span , format ! ("coercing `{expr_ty}` to `{}dyn Any`" , mutability . ref_prefix_str ()) , "consider dereferencing" , sugg . to_string () , Applicability :: MaybeIncorrect ,) ; } }
};
}
