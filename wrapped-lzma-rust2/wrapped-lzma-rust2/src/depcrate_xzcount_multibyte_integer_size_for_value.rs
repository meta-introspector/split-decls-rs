// Generated macro for count_multibyte_integer_size_for_value (function)
macro_rules! Depcrate_xzcount_multibyte_integer_size_for_value {
() => {
// Module: crate::xz
// Provides: {"count_multibyte_integer_size_for_value"}
// Dependencies: {}
fn count_multibyte_integer_size_for_value (mut value : u64) -> usize { if value == 0 { return 1 ; } let mut count = 0 ; while value > 0 { count += 1 ; value >>= 7 ; } count }
};
}
