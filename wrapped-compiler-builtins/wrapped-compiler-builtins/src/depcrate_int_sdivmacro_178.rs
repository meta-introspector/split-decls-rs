// Generated macro for macro_178 (macro)
macro_rules! Depcrate_int_sdivmacro_178 {
() => {
// Module: crate::int::sdiv
// Provides: {"macro_178"}
// Dependencies: {}
# [cfg (target_arch = "avr")] intrinsics ! { # [doc = " Returns `a / b` and `a % b` packed together."] # [doc = ""] # [doc = " Ideally we'd use `-> (u32, u32)` or some kind of a packed struct, but"] # [doc = " both force a stack allocation, while our result has to be in R18:R26."] pub extern "C" fn __divmodsi4 (a : i32 , b : i32) -> u64 { let a_neg = a < 0 ; let b_neg = b < 0 ; let mut a = a ; let mut b = b ; if a_neg { a = a . wrapping_neg () ; } if b_neg { b = b . wrapping_neg () ; } let tr = __udivmodsi4 (a as u32 , b as u32) ; let mut t = tr as u32 as i32 ; let mut r = (tr >> 32) as u32 as i32 ; if a_neg { r = r . wrapping_neg () ; } if a_neg != b_neg { t = t . wrapping_neg () ; } ((r as u32 as u64) << 32) | (t as u32 as u64) } }
};
}
