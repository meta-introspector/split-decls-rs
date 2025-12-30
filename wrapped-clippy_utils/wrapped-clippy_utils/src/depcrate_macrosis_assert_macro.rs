// Generated macro for is_assert_macro (function)
macro_rules! Depcrate_macrosis_assert_macro {
() => {
// Module: crate::macros
// Provides: {"is_assert_macro"}
// Dependencies: {}
# [doc = " Is `def_id` of `assert!` or `debug_assert!`"] pub fn is_assert_macro (cx : & LateContext < '_ > , def_id : DefId) -> bool { let Some (name) = cx . tcx . get_diagnostic_name (def_id) else { return false ; } ; matches ! (name , sym :: assert_macro | sym :: debug_assert_macro) }
};
}
