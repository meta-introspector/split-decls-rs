// Generated macro for is_panic (function)
macro_rules! Depcrate_macrosis_panic {
() => {
// Module: crate::macros
// Provides: {"is_panic"}
// Dependencies: {}
# [doc = " Is `def_id` of `std::panic`, `core::panic` or any inner implementation macros"] pub fn is_panic (cx : & LateContext < '_ > , def_id : DefId) -> bool { let Some (name) = cx . tcx . get_diagnostic_name (def_id) else { return false ; } ; matches ! (name , sym :: core_panic_macro | sym :: std_panic_macro | sym :: core_panic_2015_macro | sym :: std_panic_2015_macro | sym :: core_panic_2021_macro) }
};
}
