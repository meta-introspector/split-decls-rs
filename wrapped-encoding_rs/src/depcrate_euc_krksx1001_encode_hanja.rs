// Generated macro for ksx1001_encode_hanja (function)
macro_rules! Depcrate_euc_krksx1001_encode_hanja {
() => {
// Module: crate::euc_kr
// Provides: {"ksx1001_encode_hanja"}
// Dependencies: {}
# [cfg (feature = "fast-hanja-encode")] # [inline (always)] fn ksx1001_encode_hanja (bmp : u16) -> Option < (u8 , u8) > { if bmp < 0xF900 { ksx1001_unified_hangul_encode (bmp) } else { Some (ksx1001_compatibility_hangul_encode (bmp)) } }
};
}
