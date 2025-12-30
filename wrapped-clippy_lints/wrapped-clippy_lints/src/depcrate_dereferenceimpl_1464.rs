// Generated macro for impl_1464 (impl)
macro_rules! Depcrate_dereferenceimpl_1464 {
() => {
// Module: crate::dereference
// Provides: {"impl_1464"}
// Dependencies: {}
impl < 'tcx > Dereferencing < 'tcx > { fn check_local_usage (& mut self , cx : & LateContext < 'tcx > , e : & Expr < 'tcx > , local : HirId) { if let Some (outer_pat) = self . ref_locals . get_mut (& local) && let Some (pat) = outer_pat && ! matches ! (cx . typeck_results () . expr_adjustments (e) , [Adjustment { kind : Adjust :: Deref (_) , .. } , Adjustment { kind : Adjust :: Deref (_) , .. } , ..]) { match get_parent_expr (cx , e) { Some (Expr { kind : ExprKind :: Field (..) , .. }) => () , Some (& Expr { span , kind : ExprKind :: Unary (UnOp :: Deref , _) , .. }) if ! span . from_expansion () => { let snip = snippet_with_context (cx , e . span , span . ctxt () , ".." , & mut pat . app) . 0 ; pat . replacements . push ((span , snip . into ())) ; } , Some (parent) if ! parent . span . from_expansion () => { if cx . precedence (parent) == ExprPrecedence :: Unambiguous { * outer_pat = None ; } else { pat . always_deref = false ; let snip = snippet_with_context (cx , e . span , parent . span . ctxt () , ".." , & mut pat . app) . 0 ; pat . replacements . push ((e . span , format ! ("&{snip}"))) ; } } , _ if ! e . span . from_expansion () => { pat . always_deref = false ; let snip = snippet_with_applicability (cx , e . span , ".." , & mut pat . app) ; pat . replacements . push ((e . span , format ! ("&{snip}"))) ; } , _ => * outer_pat = None , } } } }
};
}
