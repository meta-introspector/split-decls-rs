// Generated macro for apply_lint (function)
macro_rules! Depcrate_matches_manual_ok_errapply_lint {
() => {
// Module: crate::matches::manual_ok_err
// Provides: {"apply_lint"}
// Dependencies: {}
# [doc = " Suggest replacing `expr` by `scrutinee.METHOD()`, where `METHOD` is either `ok` or"] # [doc = " `err`, depending on `is_ok`."] fn apply_lint (cx : & LateContext < '_ > , expr : & Expr < '_ > , scrutinee : & Expr < '_ > , is_ok : bool) { let method = if is_ok { "ok" } else { "err" } ; let mut app = if span_contains_comment (cx . sess () . source_map () , expr . span) { Applicability :: MaybeIncorrect } else { Applicability :: MachineApplicable } ; let scrut = Sugg :: hir_with_applicability (cx , scrutinee , ".." , & mut app) . maybe_paren () ; let scrutinee_ty = cx . typeck_results () . expr_ty (scrutinee) ; let (_ , _ , mutability) = peel_and_count_ty_refs (scrutinee_ty) ; let prefix = match mutability { Some (Mutability :: Mut) => ".as_mut()" , Some (Mutability :: Not) => ".as_ref()" , None => "" , } ; let sugg = format ! ("{scrut}{prefix}.{method}()") ; let sugg = if let Some (parent_expr) = get_parent_expr (cx , expr) && let ExprKind :: If (_ , _ , Some (else_part)) = parent_expr . kind && else_part . hir_id == expr . hir_id { reindent_multiline (& format ! ("{{\n    {sugg}\n}}") , true , indent_of (cx , parent_expr . span)) } else { sugg } ; span_lint_and_sugg (cx , MANUAL_OK_ERR , expr . span , format ! ("manual implementation of `{method}`") , "replace with" , sugg , app ,) ; }
};
}
