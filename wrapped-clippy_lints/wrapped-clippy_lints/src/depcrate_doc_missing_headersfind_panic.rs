// Generated macro for find_panic (function)
macro_rules! Depcrate_doc_missing_headersfind_panic {
() => {
// Module: crate::doc::missing_headers
// Provides: {"find_panic"}
// Dependencies: {}
fn find_panic (cx : & LateContext < '_ > , body_id : BodyId) -> Option < Span > { let mut panic_span = None ; let typeck = cx . tcx . typeck_body (body_id) ; for_each_expr (cx , cx . tcx . hir_body (body_id) , | expr | { if is_inside_always_const_context (cx . tcx , expr . hir_id) { return ControlFlow :: < ! > :: Continue (()) ; } if let Some (macro_call) = root_macro_call_first_node (cx , expr) && (is_panic (cx , macro_call . def_id) || matches ! (cx . tcx . get_diagnostic_name (macro_call . def_id) , Some (sym :: assert_macro | sym :: assert_eq_macro | sym :: assert_ne_macro))) && ! fulfill_or_allowed (cx , MISSING_PANICS_DOC , [expr . hir_id]) && panic_span . is_none () { panic_span = Some (macro_call . span) ; } if let Some (arglists) = method_chain_args (expr , & [sym :: unwrap]) . or_else (| | method_chain_args (expr , & [sym :: expect])) && let receiver_ty = typeck . expr_ty (arglists [0] . 0) . peel_refs () && matches ! (receiver_ty . opt_diag_name (cx) , Some (sym :: Option | sym :: Result)) && ! fulfill_or_allowed (cx , MISSING_PANICS_DOC , [expr . hir_id]) && panic_span . is_none () { panic_span = Some (expr . span) ; } ControlFlow :: < ! > :: Continue (()) }) ; panic_span }
};
}
