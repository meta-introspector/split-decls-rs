// Generated macro for check_method (function)
macro_rules! Depcrate_return_self_not_must_usecheck_method {
() => {
// Module: crate::return_self_not_must_use
// Provides: {"check_method"}
// Dependencies: {}
fn check_method (cx : & LateContext < '_ > , decl : & FnDecl < '_ > , fn_def : LocalDefId , span : Span , owner_id : OwnerId) { if ! span . in_external_macro (cx . sess () . source_map ()) && decl . implicit_self . has_implicit_self () && cx . effective_visibilities . is_exported (fn_def) && ! find_attr ! (cx . tcx . hir_attrs (owner_id . into ()) , AttributeKind :: MustUse { .. }) && cx . tcx . visibility (fn_def . to_def_id ()) . is_public () && let ret_ty = return_ty (cx , owner_id) && let self_arg = nth_arg (cx , owner_id , 0) && self_arg . peel_refs () == ret_ty && ! is_must_use_ty (cx , ret_ty) { span_lint_and_help (cx , RETURN_SELF_NOT_MUST_USE , span , "missing `#[must_use]` attribute on a method returning `Self`" , None , "consider adding the `#[must_use]` attribute to the method or directly to the `Self` type" ,) ; } }
};
}
