// Generated macro for used_underscore_items (function)
macro_rules! Depcrate_miscused_underscore_items {
() => {
// Module: crate::misc
// Provides: {"used_underscore_items"}
// Dependencies: {}
fn used_underscore_items < 'tcx > (cx : & LateContext < 'tcx > , expr : & 'tcx Expr < '_ >) { let (def_id , ident) = match expr . kind { ExprKind :: Call (func , ..) => { if let ExprKind :: Path (QPath :: Resolved (.. , path)) = func . kind && let Some (last_segment) = path . segments . last () && let Res :: Def (_ , def_id) = last_segment . res { (def_id , last_segment . ident) } else { return ; } } , ExprKind :: MethodCall (path , ..) => { if let Some (def_id) = cx . typeck_results () . type_dependent_def_id (expr . hir_id) { (def_id , path . ident) } else { return ; } } , ExprKind :: Struct (QPath :: Resolved (_ , path) , ..) => { if let Some (last_segment) = path . segments . last () && let Res :: Def (_ , def_id) = last_segment . res { (def_id , last_segment . ident) } else { return ; } } , _ => return , } ; let name = ident . name . as_str () ; let definition_span = cx . tcx . def_span (def_id) ; if name . starts_with ('_') && ! name . starts_with ("__") && ! definition_span . from_expansion () && def_id . is_local () && ! cx . tcx . is_foreign_item (def_id) { span_lint_and_then (cx , USED_UNDERSCORE_ITEMS , expr . span , "used underscore-prefixed item" . to_string () , | diag | { diag . span_note (definition_span , "item is defined here" . to_string ()) ; } ,) ; } }
};
}
