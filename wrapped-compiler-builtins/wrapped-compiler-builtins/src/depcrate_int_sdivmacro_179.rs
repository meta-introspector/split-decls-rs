// Generated macro for macro_179 (macro)
macro_rules! Depcrate_int_sdivmacro_179 {
() => {
// Module: crate::int::sdiv
// Provides: {"macro_179"}
// Dependencies: {}
intrinsics ! { # [maybe_use_optimized_c_shim] # [arm_aeabi_alias = __aeabi_idiv] # [doc = " Returns `n / d`"] pub extern "C" fn __divsi3 (a : i32 , b : i32) -> i32 { let a_neg = a < 0 ; let b_neg = b < 0 ; let mut a = a ; let mut b = b ; if a_neg { a = a . wrapping_neg () ; } if b_neg { b = b . wrapping_neg () ; } let t = __udivsi3 (a as u32 , b as u32) as i32 ; if a_neg != b_neg { t . wrapping_neg () } else { t } } }
};
}
