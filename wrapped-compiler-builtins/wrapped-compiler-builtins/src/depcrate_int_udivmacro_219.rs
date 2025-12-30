// Generated macro for macro_219 (macro)
macro_rules! Depcrate_int_udivmacro_219 {
() => {
// Module: crate::int::udiv
// Provides: {"macro_219"}
// Dependencies: {}
# [cfg (not (target_arch = "avr"))] intrinsics ! { # [maybe_use_optimized_c_shim] # [doc = " Returns `n / d` and sets `*rem = n % d`"] pub extern "C" fn __udivmodsi4 (n : u32 , d : u32 , rem : Option <& mut u32 >) -> u32 { let quo_rem = u32_div_rem (n , d) ; if let Some (rem) = rem { * rem = quo_rem . 1 ; } quo_rem . 0 } }
};
}
