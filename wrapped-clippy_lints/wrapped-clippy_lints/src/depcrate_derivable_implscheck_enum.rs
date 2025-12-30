// Generated macro for check_enum (function)
macro_rules! Depcrate_derivable_implscheck_enum {
() => {
// Module: crate::derivable_impls
// Provides: {"check_enum"}
// Dependencies: {}
fn check_enum < 'tcx > (cx : & LateContext < 'tcx > , item : & 'tcx Item < 'tcx > , func_expr : & 'tcx Expr < 'tcx > , adt_def : AdtDef < 'tcx > , is_const : bool ,) { if let Some (variant_def) = extract_enum_variant (cx , func_expr , adt_def) && variant_def . fields . is_empty () && ! variant_def . is_field_list_non_exhaustive () { let enum_span = cx . tcx . def_span (adt_def . did ()) ; let indent_enum = indent_of (cx , enum_span) . unwrap_or (0) ; let variant_span = cx . tcx . def_span (variant_def . def_id) ; let indent_variant = indent_of (cx , variant_span) . unwrap_or (0) ; let Some (derive_snippet) = determine_derive_macro (cx , is_const) else { return ; } ; let suggestions = vec ! [(item . span , String :: new ()) , (enum_span . shrink_to_lo () , format ! ("#[{derive_snippet}(Default)]\n{}" , " " . repeat (indent_enum)) ,) , (variant_span . shrink_to_lo () , format ! ("#[default]\n{}" , " " . repeat (indent_variant)) ,) ,] ; span_lint_and_then (cx , DERIVABLE_IMPLS , item . span , "this `impl` can be derived" , | diag | { diag . multipart_suggestion ("replace the manual implementation with a derive attribute and mark the default variant" , suggestions , Applicability :: MachineApplicable ,) ; }) ; } }
};
}
