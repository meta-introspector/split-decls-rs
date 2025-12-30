// Generated macro for ksx1001_compatibility_hangul_encode (function)
macro_rules! Depcrate_dataksx1001_compatibility_hangul_encode {
() => {
// Module: crate::data
// Provides: {"ksx1001_compatibility_hangul_encode"}
// Dependencies: {}
# [cfg (feature = "fast-hanja-encode")] # [inline (always)] pub fn ksx1001_compatibility_hangul_encode (bmp : u16) -> (u8 , u8) { let pair = & KSX1001_COMPATIBILITY_HANJA_BYTES [bmp as usize - 0xF900] ; (pair [0] , pair [1]) }
};
}
