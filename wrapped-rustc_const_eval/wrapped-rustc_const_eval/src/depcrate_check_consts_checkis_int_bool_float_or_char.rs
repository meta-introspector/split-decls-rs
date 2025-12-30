// Generated macro for is_int_bool_float_or_char (function)
macro_rules! Depcrate_check_consts_checkis_int_bool_float_or_char {
() => {
// Module: crate::check_consts::check
// Provides: {"is_int_bool_float_or_char"}
// Dependencies: {}
fn is_int_bool_float_or_char (ty : Ty < '_ >) -> bool { ty . is_bool () || ty . is_integral () || ty . is_char () || ty . is_floating_point () }
};
}
