// Generated macro for check_must_use_candidate (function)
macro_rules! Depcrate_functions_must_usecheck_must_use_candidate {
() => {
// Module: crate::functions::must_use
// Provides: {"check_must_use_candidate"}
// Dependencies: {}
fn check_must_use_candidate < 'tcx > (cx : & LateContext < 'tcx > , decl : & 'tcx hir :: FnDecl < '_ > , body : & 'tcx hir :: Body < '_ > , item_span : Span , ident_span : Span , item_id : hir :: OwnerId , msg : & 'static str ,) { if has_mutable_arg (cx , body) || mutates_static (cx , body) || item_span . in_external_macro (cx . sess () . source_map ()) || returns_unit (decl) || ! cx . effective_visibilities . is_exported (item_id . def_id) || is_must_use_ty (cx , return_ty (cx , item_id)) || item_span . from_expansion () { return ; } span_lint_and_then (cx , MUST_USE_CANDIDATE , ident_span , msg , | diag | { let indent = snippet_indent (cx , item_span) . unwrap_or_default () ; diag . span_suggestion (item_span . shrink_to_lo () , "add the attribute" , format ! ("#[must_use] \n{indent}") , Applicability :: MachineApplicable ,) ; }) ; }
};
}
