// Generated macro for check_fn (function)
macro_rules! Depcrate_functions_impl_trait_in_paramscheck_fn {
() => {
// Module: crate::functions::impl_trait_in_params
// Provides: {"check_fn"}
// Dependencies: {}
pub (super) fn check_fn < 'tcx > (cx : & LateContext < '_ > , kind : & 'tcx FnKind < '_ > , body : & 'tcx Body < '_ > , hir_id : HirId) { if let FnKind :: ItemFn (_ , generics , _) = kind && cx . tcx . visibility (cx . tcx . hir_body_owner_def_id (body . id ())) . is_public () && ! is_in_test (cx . tcx , hir_id) { for param in generics . params { if param . is_impl_trait () { report (cx , param , generics) ; } } } }
};
}
