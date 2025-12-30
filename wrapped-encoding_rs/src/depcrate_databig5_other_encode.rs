// Generated macro for big5_other_encode (function)
macro_rules! Depcrate_databig5_other_encode {
() => {
// Module: crate::data
// Provides: {"big5_other_encode"}
// Dependencies: {}
# [cfg (feature = "fast-big5-hanzi-encode")] # [inline (always)] pub fn big5_other_encode (bmp : u16) -> Option < usize > { if 0x4491 == bmp { return Some (11209) ; } if 0xFA0D == bmp { return Some (14598) ; } if 0xFA0C == bmp { return Some (11314) ; } if let Some (pos) = position (& BIG5_LOW_BITS [(5024 - 942) .. (5466 - 942)] , bmp) { return Some (pos + 5024) ; } if let Some (pos) = position (& BIG5_LOW_BITS [(10896 - 942) .. (11205 - 942)] , bmp) { return Some (pos + 10896) ; } if let Some (pos) = position (& BIG5_LOW_BITS [(11254 - 942) .. (11304 - 942)] , bmp) { return Some (pos + 11254) ; } let mut i = 18996 - 942 ; while i < BIG5_LOW_BITS . len () { if BIG5_LOW_BITS [i] == bmp && ! big5_is_astral (i) { return Some (i + 942) ; } i += 1 ; } None }
};
}
