// Generated macro for macro_206 (macro)
macro_rules! Depcrate_int_trailing_zerosmacro_206 {
() => {
// Module: crate::int::trailing_zeros
// Provides: {"macro_206"}
// Dependencies: {}
intrinsics ! { # [doc = " Returns the number of trailing binary zeros in `x` (32 bit version)."] pub extern "C" fn __ctzsi2 (x : u32) -> usize { trailing_zeros (x) } # [doc = " Returns the number of trailing binary zeros in `x` (64 bit version)."] pub extern "C" fn __ctzdi2 (x : u64) -> usize { trailing_zeros (x) } # [doc = " Returns the number of trailing binary zeros in `x` (128 bit version)."] pub extern "C" fn __ctzti2 (x : u128) -> usize { let lo = x as u64 ; if lo == 0 { 64 + __ctzdi2 ((x >> 64) as u64) } else { __ctzdi2 (lo) } } }
};
}
