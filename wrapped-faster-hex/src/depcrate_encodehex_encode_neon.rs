// Generated macro for hex_encode_neon (function)
macro_rules! Depcrate_encodehex_encode_neon {
() => {
// Module: crate::encode
// Provides: {"hex_encode_neon"}
// Dependencies: {}
# [target_feature (enable = "neon")] # [cfg (target_arch = "aarch64")] unsafe fn hex_encode_neon (mut src : & [u8] , dst : & mut [u8] , upper_case : bool) { let ascii_zero = vdupq_n_u8 (b'0') ; let nines = vdupq_n_u8 (9) ; let ascii_a = if upper_case { vdupq_n_u8 (b'A' - 9 - 1) } else { vdupq_n_u8 (b'a' - 9 - 1) } ; let and4bits = vdupq_n_u8 (0xf) ; let mut i = 0_isize ; while src . len () >= 16 { let invec = vld1q_u8 (src . as_ptr () as * const _) ; let masked1 = vandq_u8 (invec , and4bits) ; let masked2 = vandq_u8 (vshrq_n_u8 :: < 4 > (invec) , and4bits) ; let cmpmask1 = vcgtq_u8 (masked1 , nines) ; let cmpmask2 = vcgtq_u8 (masked2 , nines) ; let masked1 = vaddq_u8 (masked1 , vbslq_u8 (cmpmask1 , ascii_a , ascii_zero)) ; let masked2 = vaddq_u8 (masked2 , vbslq_u8 (cmpmask2 , ascii_a , ascii_zero)) ; let res1 = vzip1q_u8 (masked2 , masked1) ; let res2 = vzip2q_u8 (masked2 , masked1) ; vst1q_u8 (dst . as_mut_ptr () . offset (i * 2) as * mut _ , res1) ; vst1q_u8 (dst . as_mut_ptr () . offset (i * 2 + 16) as * mut _ , res2) ; src = & src [16 ..] ; i += 16 ; } let i = i as usize ; hex_encode_custom_case_fallback (src , & mut dst [i * 2 ..] , upper_case) ; }
};
}
