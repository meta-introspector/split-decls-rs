// Generated macro for is_visible_outside_module (function)
macro_rules! Depcrate_error_impl_erroris_visible_outside_module {
() => {
// Module: crate::error_impl_error
// Provides: {"is_visible_outside_module"}
// Dependencies: {}
# [doc = " Do not lint private `Error`s, i.e., ones without any `pub` (minus `pub(self)` of course) and"] # [doc = " which aren't reexported"] fn is_visible_outside_module (cx : & LateContext < '_ > , def_id : LocalDefId) -> bool { ! matches ! (cx . tcx . visibility (def_id) , Visibility :: Restricted (mod_def_id) if cx . tcx . parent_module_from_def_id (def_id) . to_def_id () == mod_def_id) }
};
}
