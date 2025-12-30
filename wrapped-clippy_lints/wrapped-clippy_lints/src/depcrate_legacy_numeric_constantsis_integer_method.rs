// Generated macro for is_integer_method (function)
macro_rules! Depcrate_legacy_numeric_constantsis_integer_method {
() => {
// Module: crate::legacy_numeric_constants
// Provides: {"is_integer_method"}
// Dependencies: {}
fn is_integer_method (cx : & LateContext < '_ > , did : DefId) -> bool { matches ! (cx . tcx . get_diagnostic_name (did) , Some (sym :: isize_legacy_fn_max_value | sym :: isize_legacy_fn_min_value | sym :: i128_legacy_fn_max_value | sym :: i128_legacy_fn_min_value | sym :: i16_legacy_fn_max_value | sym :: i16_legacy_fn_min_value | sym :: i32_legacy_fn_max_value | sym :: i32_legacy_fn_min_value | sym :: i64_legacy_fn_max_value | sym :: i64_legacy_fn_min_value | sym :: i8_legacy_fn_max_value | sym :: i8_legacy_fn_min_value | sym :: usize_legacy_fn_max_value | sym :: usize_legacy_fn_min_value | sym :: u128_legacy_fn_max_value | sym :: u128_legacy_fn_min_value | sym :: u16_legacy_fn_max_value | sym :: u16_legacy_fn_min_value | sym :: u32_legacy_fn_max_value | sym :: u32_legacy_fn_min_value | sym :: u64_legacy_fn_max_value | sym :: u64_legacy_fn_min_value | sym :: u8_legacy_fn_max_value | sym :: u8_legacy_fn_min_value)) }
};
}
