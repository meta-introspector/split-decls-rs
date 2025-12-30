// Generated macro for impl_10735 (impl)
macro_rules! Depcrate_unused_peekableimpl_10735 {
() => {
// Module: crate::unused_peekable
// Provides: {"impl_10735"}
// Dependencies: {}
impl < 'tcx > LateLintPass < 'tcx > for UnusedPeekable { fn check_block (& mut self , cx : & LateContext < 'tcx > , block : & Block < 'tcx >) { if let Some (expr) = block . expr && let Some (ty) = cx . typeck_results () . expr_ty_opt (peel_ref_operators (cx , expr)) && is_type_diagnostic_item (cx , ty , sym :: IterPeekable) { return ; } for (idx , stmt) in block . stmts . iter () . enumerate () { if ! stmt . span . from_expansion () && let StmtKind :: Let (local) = stmt . kind && let PatKind :: Binding (_ , binding , ident , _) = local . pat . kind && let Some (init) = local . init && ! init . span . from_expansion () && let Some (ty) = cx . typeck_results () . expr_ty_opt (init) && let (ty , _ , Mutability :: Mut) = peel_mid_ty_refs_is_mutable (ty) && is_type_diagnostic_item (cx , ty , sym :: IterPeekable) { let mut vis = PeekableVisitor :: new (cx , binding) ; if idx + 1 == block . stmts . len () && block . expr . is_none () { return ; } let mut found_peek_call = block . stmts [idx ..] . iter () . any (| stmt | vis . visit_stmt (stmt) . is_break ()) ; if ! found_peek_call && let Some (expr) = block . expr && vis . visit_expr (expr) . is_break () { found_peek_call = true ; } if ! found_peek_call { span_lint_hir_and_then (cx , UNUSED_PEEKABLE , local . hir_id , ident . span , "`peek` never called on `Peekable` iterator" , | diag | { diag . help ("consider removing the call to `peekable`") ; } ,) ; } } } } }
};
}
