macro_rules! macro_43 {
    () => {
        cfg_if ! { if # [cfg (target_feature = "sse2")] { # [inline (always)] pub fn simd_pack (a : u16x8 , b : u16x8) -> u8x16 { unsafe { _mm_packus_epi16 (a . into () , b . into ()) . into () } } } else { # [inline (always)] pub fn simd_pack (a : u16x8 , b : u16x8) -> u8x16 { let first : u8x16 = a . to_ne_bytes () ; let second : u8x16 = b . to_ne_bytes () ; simd_swizzle ! (first , second , [0 , 2 , 4 , 6 , 8 , 10 , 12 , 14 , 16 , 18 , 20 , 22 , 24 , 26 , 28 , 30]) } } }
    };
}

macro_43!();