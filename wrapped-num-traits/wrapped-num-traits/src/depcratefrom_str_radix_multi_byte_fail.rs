// Generated macro for from_str_radix_multi_byte_fail (function)
macro_rules! Depcratefrom_str_radix_multi_byte_fail {
() => {
// Module: crate
// Provides: {"from_str_radix_multi_byte_fail"}
// Dependencies: {}
# [test] fn from_str_radix_multi_byte_fail () { assert ! (f32 :: from_str_radix ("™0.2" , 10) . is_err ()) ; assert ! (f32 :: from_str_radix ("0.2E™1" , 10) . is_err ()) ; }
};
}
