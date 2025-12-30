// Generated macro for is_integer_module (function)
macro_rules! Depcrate_legacy_numeric_constantsis_integer_module {
() => {
// Module: crate::legacy_numeric_constants
// Provides: {"is_integer_module"}
// Dependencies: {}
fn is_integer_module (cx : & LateContext < '_ > , did : DefId) -> bool { matches ! (cx . tcx . get_diagnostic_name (did) , Some (sym :: isize_legacy_mod | sym :: i128_legacy_mod | sym :: i64_legacy_mod | sym :: i32_legacy_mod | sym :: i16_legacy_mod | sym :: i8_legacy_mod | sym :: usize_legacy_mod | sym :: u128_legacy_mod | sym :: u64_legacy_mod | sym :: u32_legacy_mod | sym :: u16_legacy_mod | sym :: u8_legacy_mod)) }
};
}
