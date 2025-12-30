// Generated macro for ksx1001_encode_hangul (function)
macro_rules! Depcrate_euc_krksx1001_encode_hangul {
() => {
// Module: crate::euc_kr
// Provides: {"ksx1001_encode_hangul"}
// Dependencies: {}
# [cfg (feature = "fast-hangul-encode")] # [inline (always)] fn ksx1001_encode_hangul (_ : u16 , bmp_minus_hangul_start : u16) -> (u8 , u8) { cp949_hangul_encode (bmp_minus_hangul_start) }
};
}
