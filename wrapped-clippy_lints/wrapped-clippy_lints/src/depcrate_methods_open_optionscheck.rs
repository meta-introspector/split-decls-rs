// Generated macro for check (function)
macro_rules! Depcrate_methods_open_optionscheck {
() => {
// Module: crate::methods::open_options
// Provides: {"check"}
// Dependencies: {}
pub (super) fn check < 'tcx > (cx : & LateContext < 'tcx > , e : & 'tcx Expr < '_ > , recv : & 'tcx Expr < '_ >) { if let Some (method_id) = cx . typeck_results () . type_dependent_def_id (e . hir_id) && let Some (impl_id) = cx . tcx . impl_of_assoc (method_id) && is_open_options (cx , cx . tcx . type_of (impl_id) . instantiate_identity ()) { let mut options = Vec :: new () ; if get_open_options (cx , recv , & mut options) { check_open_options (cx , & options , e . span) ; } } }
};
}
