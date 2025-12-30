// Generated macro for macro_157 (macro)
macro_rules! Depcrate_int_leading_zerosmacro_157 {
() => {
// Module: crate::int::leading_zeros
// Provides: {"macro_157"}
// Dependencies: {}
intrinsics ! { # [doc = " Returns the number of leading binary zeros in `x`"] pub extern "C" fn __clzsi2 (x : u32) -> usize { if cfg ! (any (target_arch = "riscv32" , target_arch = "riscv64")) { leading_zeros_riscv (x) } else { leading_zeros_default (x) } } # [doc = " Returns the number of leading binary zeros in `x`"] pub extern "C" fn __clzdi2 (x : u64) -> usize { if cfg ! (any (target_arch = "riscv32" , target_arch = "riscv64")) { leading_zeros_riscv (x) } else { leading_zeros_default (x) } } # [doc = " Returns the number of leading binary zeros in `x`"] pub extern "C" fn __clzti2 (x : u128) -> usize { let hi = (x >> 64) as u64 ; if hi == 0 { 64 + __clzdi2 (x as u64) } else { __clzdi2 (hi) } } }
};
}
