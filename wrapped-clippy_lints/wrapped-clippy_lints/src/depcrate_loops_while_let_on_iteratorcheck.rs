// Generated macro for check (function)
macro_rules! Depcrate_loops_while_let_on_iteratorcheck {
() => {
// Module: crate::loops::while_let_on_iterator
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { if let Some (higher :: WhileLet { if_then , let_pat , let_expr , label , .. }) = higher :: WhileLet :: hir (expr) && let Some (some_pat) = as_some_pattern (cx , let_pat) && let ExprKind :: MethodCall (method_name , iter_expr , [] , _) = let_expr . kind && method_name . ident . name == sym :: next && cx . ty_based_def (let_expr) . opt_parent (cx) . is_diag_item (cx , sym :: Iterator) && let Some (iter_expr_struct) = try_parse_iter_expr (cx , iter_expr) && ! uses_iter (cx , & iter_expr_struct , if_then) { let mut applicability = Applicability :: MachineApplicable ; let loop_label = label . map_or (String :: new () , | l | format ! ("{}: " , l . ident . name)) ; let loop_var = if let Some (some_pat) = some_pat . first () { if is_refutable (cx , some_pat) { return ; } snippet_with_applicability (cx , some_pat . span , ".." , & mut applicability) } else { "_" . into () } ; let by_ref = if cx . typeck_results () . expr_ty (iter_expr) . ref_mutability () == Some (Mutability :: Mut) || ! iter_expr_struct . can_move || ! iter_expr_struct . fields . is_empty () || needs_mutable_borrow (cx , & iter_expr_struct , expr) { ".by_ref()" } else { "" } ; let iterator = snippet_with_applicability (cx , iter_expr . span , "_" , & mut applicability) ; span_lint_and_sugg (cx , WHILE_LET_ON_ITERATOR , expr . span . with_hi (let_expr . span . hi ()) , "this loop could be written as a `for` loop" , "try" , format ! ("{loop_label}for {loop_var} in {iterator}{by_ref}") , applicability ,) ; } }
};
}
