// Generated macro for check_struct (function)
macro_rules! Depcrate_missing_fields_in_debugcheck_struct {
() => {
// Module: crate::missing_fields_in_debug
// Provides: {"check_struct"}
// Dependencies: {}
# [doc = " Attempts to find unused fields assuming that the item is a struct"] fn check_struct < 'tcx > (cx : & LateContext < 'tcx > , typeck_results : & TypeckResults < 'tcx > , block : & 'tcx Block < 'tcx > , self_ty : Ty < 'tcx > , item : & 'tcx Item < 'tcx > , data : & VariantData < '_ > ,) { let mut has_direct_field_access = false ; let mut field_accesses = FxHashSet :: default () ; for_each_expr (cx , block , | expr | { if let ExprKind :: Field (target , ident) = expr . kind && let target_ty = typeck_results . expr_ty_adjusted (target) . peel_refs () && target_ty == self_ty { field_accesses . insert (ident . name) ; has_direct_field_access = true ; } else if let Some (sym) = as_field_call (cx , typeck_results , expr) { field_accesses . insert (sym) ; } ControlFlow :: < ! , _ > :: Continue (()) }) ; let span_notes = data . fields () . iter () . filter_map (| field | { if field_accesses . contains (& field . ident . name) || field . ty . basic_res () . is_lang_item (cx , LangItem :: PhantomData) { None } else { Some ((field . span , "this field is unused")) } }) . collect :: < Vec < _ > > () ; if ! span_notes . is_empty () && has_direct_field_access { report_lints (cx , item . span , span_notes) ; } }
};
}
