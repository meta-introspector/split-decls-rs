// Generated macro for get_mask_u64 (function)
macro_rules! Depcrate_neonget_mask_u64 {
() => {
// Module: crate::neon
// Provides: {"get_mask_u64"}
// Dependencies: {}
# [doc = " Moves a mask created by `vceqq_u8` to a `u64` register, with each all-zero or"] # [doc = " all-ones mask byte represented as an all-zero or all-ones half-byte."] # [must_use] # [inline (always)] fn get_mask_u64 (mask : uint8x16_t) -> u64 { unsafe { let mask = vshrn_n_u16_4_hide (vreinterpretq_u16_u8 (mask)) ; vget_lane_u64 (vreinterpret_u64_u8 (mask) , 0) } }
};
}
