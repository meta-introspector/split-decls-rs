// Generated macro for check_needless_must_use (function)
macro_rules! Depcrate_functions_must_usecheck_needless_must_use {
() => {
// Module: crate::functions::must_use
// Provides: {"check_needless_must_use"}
// Dependencies: {}
# [expect (clippy :: too_many_arguments)] fn check_needless_must_use (cx : & LateContext < '_ > , decl : & hir :: FnDecl < '_ > , item_id : hir :: OwnerId , item_span : Span , fn_header_span : Span , attr_span : Span , reason : Option < Symbol > , attrs : & [Attribute] , sig : & FnSig < '_ > ,) { if item_span . in_external_macro (cx . sess () . source_map ()) { return ; } if returns_unit (decl) { if attrs . len () == 1 { span_lint_and_then (cx , MUST_USE_UNIT , fn_header_span , "this unit-returning function has a `#[must_use]` attribute" , | diag | { diag . span_suggestion (attr_span , "remove the attribute" , "" , Applicability :: MachineApplicable) ; } ,) ; } else { span_lint_and_help (cx , MUST_USE_UNIT , fn_header_span , "this unit-returning function has a `#[must_use]` attribute" , Some (attr_span) , "remove `must_use`" ,) ; } } else if reason . is_none () && is_must_use_ty (cx , return_ty (cx , item_id)) { if sig . header . is_async () { let infcx = cx . tcx . infer_ctxt () . build (cx . typing_mode ()) ; if let Some (future_ty) = infcx . err_ctxt () . get_impl_future_output_ty (return_ty (cx , item_id)) && ! is_must_use_ty (cx , future_ty) { return ; } } span_lint_and_help (cx , DOUBLE_MUST_USE , fn_header_span , "this function has a `#[must_use]` attribute with no message, but returns a type already marked as `#[must_use]`" , None , "either add some descriptive message or remove the attribute" ,) ; } }
};
}
