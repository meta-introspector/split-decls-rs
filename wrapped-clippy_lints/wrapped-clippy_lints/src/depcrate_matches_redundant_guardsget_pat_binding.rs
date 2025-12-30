// Generated macro for get_pat_binding (function)
macro_rules! Depcrate_matches_redundant_guardsget_pat_binding {
() => {
// Module: crate::matches::redundant_guards
// Provides: {"get_pat_binding"}
// Dependencies: {}
fn get_pat_binding < 'tcx > (cx : & LateContext < 'tcx > , guard_expr : & Expr < '_ > , outer_arm : & Arm < 'tcx > ,) -> Option < PatBindingInfo > { if let Some (local) = guard_expr . res_local_id () && ! is_local_used (cx , outer_arm . body , local) { let mut span = None ; let mut byref_ident = None ; let mut multiple_bindings = false ; outer_arm . pat . walk (| pat | { if let PatKind :: Binding (bind_annot , hir_id , ident , _) = pat . kind && hir_id == local { if matches ! (bind_annot . 0 , rustc_ast :: ByRef :: Yes (..)) { let _ = byref_ident . insert (ident) ; } if span . replace (pat . span) . is_some () { multiple_bindings = true ; return false ; } } true }) ; if ! multiple_bindings { return span . map (| span | PatBindingInfo { span , byref_ident , is_field : matches ! (cx . tcx . parent_hir_node (local) , Node :: PatField (_)) , }) ; } } None }
};
}
