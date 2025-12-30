// Generated macro for ksx1001_unified_hangul_encode (function)
macro_rules! Depcrate_dataksx1001_unified_hangul_encode {
() => {
// Module: crate::data
// Provides: {"ksx1001_unified_hangul_encode"}
// Dependencies: {}
# [cfg (feature = "fast-hanja-encode")] # [inline (always)] pub fn ksx1001_unified_hangul_encode (bmp : u16) -> Option < (u8 , u8) > { let pair = & KSX1001_UNIFIED_HANJA_BYTES [bmp as usize - 0x4E00] ; if pair [0] == 0 && pair [1] == 0 { return None ; } Some ((pair [0] , pair [1])) }
};
}
