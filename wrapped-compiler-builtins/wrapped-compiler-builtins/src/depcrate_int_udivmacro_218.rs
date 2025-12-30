// Generated macro for macro_218 (macro)
macro_rules! Depcrate_int_udivmacro_218 {
() => {
// Module: crate::int::udiv
// Provides: {"macro_218"}
// Dependencies: {}
intrinsics ! { # [maybe_use_optimized_c_shim] # [arm_aeabi_alias = __aeabi_uidiv] # [doc = " Returns `n / d`"] pub extern "C" fn __udivsi3 (n : u32 , d : u32) -> u32 { u32_div_rem (n , d) . 0 } # [maybe_use_optimized_c_shim] # [doc = " Returns `n % d`"] pub extern "C" fn __umodsi3 (n : u32 , d : u32) -> u32 { u32_div_rem (n , d) . 1 } }
};
}
