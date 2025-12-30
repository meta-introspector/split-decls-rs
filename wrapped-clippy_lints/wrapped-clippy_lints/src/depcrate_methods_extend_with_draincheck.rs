// Generated macro for check (function)
macro_rules! Depcrate_methods_extend_with_draincheck {
() => {
// Module: crate::methods::extend_with_drain
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check (cx : & LateContext < '_ > , expr : & Expr < '_ > , recv : & Expr < '_ > , arg : & Expr < '_ >) { let ty = cx . typeck_results () . expr_ty (recv) . peel_refs () ; if ty . is_diag_item (cx , sym :: Vec) && let ExprKind :: MethodCall (src_method , drain_vec , [drain_arg] , _) = & arg . kind && src_method . ident . name == sym :: drain && let src_ty = cx . typeck_results () . expr_ty (drain_vec) && let immutable = src_ty . is_mutable_ptr () && let src_ty = src_ty . peel_refs () && src_ty . is_diag_item (cx , sym :: Vec) && let src_ty_range = cx . typeck_results () . expr_ty (drain_arg) . peel_refs () && src_ty_range . is_lang_item (cx , LangItem :: RangeFull) { let mut applicability = Applicability :: MachineApplicable ; span_lint_and_sugg (cx , EXTEND_WITH_DRAIN , expr . span , "use of `extend` instead of `append` for adding the full range of a second vector" , "try" , format ! ("{}.append({}{})" , snippet_with_applicability (cx , recv . span , ".." , & mut applicability) , if immutable { "" } else { "&mut " } , snippet_with_applicability (cx , drain_vec . span , ".." , & mut applicability)) , applicability ,) ; } }
};
}
