// Generated macro for check_impl_item (function)
macro_rules! Depcrate_methods_should_implement_traitcheck_impl_item {
() => {
// Module: crate::methods::should_implement_trait
// Provides: {"check_impl_item"}
// Dependencies: {}
pub (super) fn check_impl_item < 'tcx > (cx : & LateContext < 'tcx > , impl_item : & 'tcx ImplItem < '_ > , self_ty : Ty < 'tcx > , impl_implements_trait : bool , first_arg_ty_opt : Option < Ty < 'tcx > > , sig : & FnSig < '_ > ,) { if ! impl_implements_trait && cx . effective_visibilities . is_exported (impl_item . owner_id . def_id) && let Some (method_config) = TRAIT_METHODS . iter () . find (| case | case . method_name == impl_item . ident . name) && sig . decl . inputs . len () == method_config . param_count && method_config . output_type . matches (& sig . decl . output) && first_arg_ty_opt . is_none_or (| first_arg_ty | method_config . self_kind . matches (cx , self_ty , first_arg_ty)) && sig . header . is_safe () && ! sig . header . is_const () && ! sig . header . is_async () && sig . header . abi == ExternAbi :: Rust && method_config . lifetime_param_cond (impl_item) && method_config . in_prelude_since <= cx . tcx . sess . edition () { span_lint_and_help (cx , SHOULD_IMPLEMENT_TRAIT , impl_item . span , format ! ("method `{}` can be confused for the standard trait method `{}::{}`" , method_config . method_name , method_config . trait_name , method_config . method_name) , None , format ! ("consider implementing the trait `{}` or choosing a less ambiguous method name" , method_config . trait_name) ,) ; } }
};
}
