macro_rules! macro_36 {
    () => {
        cfg_if ! { if # [cfg (target_feature = "sse2")] { # [inline (always)] pub fn simd_is_str_latin1 (s : u8x16) -> bool { if simd_is_ascii (s) { return true ; } let above_str_latin1 = u8x16 :: splat (0xC4) ; s . simd_lt (above_str_latin1) . all () } } else if # [cfg (target_arch = "aarch64")] { # [inline (always)] pub fn simd_is_str_latin1 (s : u8x16) -> bool { unsafe { vmaxvq_u8 (s . into ()) < 0xC4 } } } else { # [inline (always)] pub fn simd_is_str_latin1 (s : u8x16) -> bool { let above_str_latin1 = u8x16 :: splat (0xC4) ; all_mask8x16 (s . simd_lt (above_str_latin1)) } } }
    };
}

macro_36!()