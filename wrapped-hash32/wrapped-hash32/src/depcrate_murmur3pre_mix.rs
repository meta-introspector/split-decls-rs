// Generated macro for pre_mix (function)
macro_rules! Depcrate_murmur3pre_mix {
() => {
// Module: crate::murmur3
// Provides: {"pre_mix"}
// Dependencies: {}
fn pre_mix (mut block : u32) -> u32 { block = block . wrapping_mul (C1) ; block = block . rotate_left (R1) ; block = block . wrapping_mul (C2) ; block }
};
}
