// Generated macro for used_underscore_binding (function)
macro_rules! Depcrate_miscused_underscore_binding {
() => {
// Module: crate::misc
// Provides: {"used_underscore_binding"}
// Dependencies: {}
fn used_underscore_binding < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let (definition_hir_id , ident) = match expr . kind { ExprKind :: Path (ref qpath) => { if let QPath :: Resolved (None , path) = qpath && let Res :: Local (id) = path . res && is_used (cx , expr) { (id , last_path_segment (qpath) . ident) } else { return ; } } , ExprKind :: Field (recv , ident) => { if let Some (adt_def) = cx . typeck_results () . expr_ty_adjusted (recv) . ty_adt_def () && let Some (field) = adt_def . all_fields () . find (| field | field . name == ident . name) && let Some (local_did) = field . did . as_local () && ! cx . tcx . type_of (field . did) . skip_binder () . is_phantom_data () { (cx . tcx . local_def_id_to_hir_id (local_did) , ident) } else { return ; } } , _ => return , } ; let name = ident . name . as_str () ; if name . starts_with ('_') && ! name . starts_with ("__") && let definition_span = cx . tcx . hir_span (definition_hir_id) && ! definition_span . from_expansion () && ! fulfill_or_allowed (cx , USED_UNDERSCORE_BINDING , [expr . hir_id , definition_hir_id]) { span_lint_and_then (cx , USED_UNDERSCORE_BINDING , expr . span , "used underscore-prefixed binding" . to_string () , | diag | { diag . span_note (definition_span , "binding is defined here" . to_string ()) ; } ,) ; } }
};
}
