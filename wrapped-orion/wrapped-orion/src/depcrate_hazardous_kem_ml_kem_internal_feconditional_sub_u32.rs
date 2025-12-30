// Generated macro for conditional_sub_u32 (function)
macro_rules! Depcrate_hazardous_kem_ml_kem_internal_feconditional_sub_u32 {
() => {
// Module: crate::hazardous::kem::ml_kem::internal::fe
// Provides: {"conditional_sub_u32"}
// Dependencies: {}
fn conditional_sub_u32 (a : u32) -> u32 { let t : u32 = a . overflowing_sub (KYBER_Q) . 0 ; let mask : u32 = 0u32 . overflowing_sub (t >> 31) . 0 ; (t & ! mask) | (a & mask) }
};
}
