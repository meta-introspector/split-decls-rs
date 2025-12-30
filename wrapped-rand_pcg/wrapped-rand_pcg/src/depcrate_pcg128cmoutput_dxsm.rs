// Generated macro for output_dxsm (function)
macro_rules! Depcrate_pcg128cmoutput_dxsm {
() => {
// Module: crate::pcg128cm
// Provides: {"output_dxsm"}
// Dependencies: {}
# [inline (always)] fn output_dxsm (state : u128) -> u64 { let mut hi = (state >> 64) as u64 ; let mut lo = state as u64 ; lo |= 1 ; hi ^= hi >> 32 ; hi = hi . wrapping_mul (MULTIPLIER) ; hi ^= hi >> 48 ; hi = hi . wrapping_mul (lo) ; hi }
};
}
