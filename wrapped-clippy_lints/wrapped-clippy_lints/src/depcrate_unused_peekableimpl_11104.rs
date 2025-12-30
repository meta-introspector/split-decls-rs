// Generated macro for impl_11104 (impl)
macro_rules! Depcrate_unused_peekableimpl_11104 {
() => {
// Module: crate::unused_peekable
// Provides: {"impl_11104"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnusedPeekable { fn check_block (& mut self , cx : & LateContext < 'tcx > , block : & Block < 'tcx >) { if let Some (expr) = block . expr && let Some (ty) = cx . typeck_results () . expr_ty_opt (peel_ref_operators (cx , expr)) && ty . is_diag_item (cx , sym :: IterPeekable) { return ; } for (idx , stmt) in block . stmts . iter () . enumerate () { if ! stmt . span . from_expansion () && let StmtKind :: Let (local) = stmt . kind && let PatKind :: Binding (_ , binding , ident , _) = local . pat . kind && let Some (init) = local . init && ! init . span . from_expansion () && let Some (ty) = cx . typeck_results () . expr_ty_opt (init) && let (ty , _ , None | Some (Mutability :: Mut)) = peel_and_count_ty_refs (ty) && ty . is_diag_item (cx , sym :: IterPeekable) { let mut vis = PeekableVisitor :: new (cx , binding) ; if idx + 1 == block . stmts . len () && block . expr . is_none () { return ; } let mut found_peek_call = block . stmts [idx ..] . iter () . any (| stmt | vis . visit_stmt (stmt) . is_break ()) ; if ! found_peek_call && let Some (expr) = block . expr && vis . visit_expr (expr) . is_break () { found_peek_call = true ; } if ! found_peek_call { span_lint_hir_and_then (cx , UNUSED_PEEKABLE , local . hir_id , ident . span , "`peek` never called on `Peekable` iterator" , | diag | { diag . help ("consider removing the call to `peekable`") ; } ,) ; } } } } }
};
}
