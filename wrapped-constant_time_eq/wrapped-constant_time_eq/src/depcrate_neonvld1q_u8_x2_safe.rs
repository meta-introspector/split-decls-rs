// Generated macro for vld1q_u8_x2_safe (function)
macro_rules! Depcrate_neonvld1q_u8_x2_safe {
() => {
// Module: crate::neon
// Provides: {"vld1q_u8_x2_safe"}
// Dependencies: {}
# [doc = " Safe equivalent to `vld1q_u8_x2` for byte slices."] # [must_use] # [inline (always)] fn vld1q_u8_x2_safe (src : & [u8]) -> uint8x16x2_t { assert_eq ! (src . len () , size_of ::< uint8x16x2_t > ()) ; unsafe { vld1q_u8_x2 (src . as_ptr ()) } }
};
}
