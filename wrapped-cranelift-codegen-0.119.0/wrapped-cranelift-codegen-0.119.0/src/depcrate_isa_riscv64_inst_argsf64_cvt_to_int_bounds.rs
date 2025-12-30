// Generated macro for f64_cvt_to_int_bounds (function)
macro_rules! Depcrate_isa_riscv64_inst_argsf64_cvt_to_int_bounds {
() => {
// Module: crate::isa::riscv64::inst::args
// Provides: {"f64_cvt_to_int_bounds"}
// Dependencies: {}
pub (crate) fn f64_cvt_to_int_bounds (signed : bool , out_bits : u32) -> (f64 , f64) { match (signed , out_bits) { (true , 8) => (i8 :: min_value () as f64 - 1. , i8 :: max_value () as f64 + 1.) , (true , 16) => (i16 :: min_value () as f64 - 1. , i16 :: max_value () as f64 + 1.) , (true , 32) => (- 2147483649.0 , 2147483648.0) , (true , 64) => (- 9223372036854777856.0 , 9223372036854775808.0) , (false , 8) => (- 1. , u8 :: max_value () as f64 + 1.) , (false , 16) => (- 1. , u16 :: max_value () as f64 + 1.) , (false , 32) => (- 1. , 4294967296.0) , (false , 64) => (- 1. , 18446744073709551616.0) , _ => unreachable ! () , } }
};
}
