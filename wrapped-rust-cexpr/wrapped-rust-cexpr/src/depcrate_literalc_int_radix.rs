// Generated macro for c_int_radix (function)
macro_rules! Depcrate_literalc_int_radix {
() => {
// Module: crate::literal
// Provides: {"c_int_radix"}
// Dependencies: {}
fn c_int_radix (n : Vec < u8 > , radix : u32) -> Option < u64 > { str :: from_utf8 (& n) . ok () . and_then (| i | u64 :: from_str_radix (i , radix) . ok ()) }
};
}
