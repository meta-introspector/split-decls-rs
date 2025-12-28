macro_rules! macro_37 {
    () => {
        cfg_if ! { if # [cfg (target_arch = "aarch64")] { # [inline (always)] pub fn simd_is_basic_latin (s : u16x8) -> bool { unsafe { vmaxvq_u16 (s . into ()) < 0x80 } } # [inline (always)] pub fn simd_is_latin1 (s : u16x8) -> bool { unsafe { vmaxvq_u16 (s . into ()) < 0x100 } } } else { # [inline (always)] pub fn simd_is_basic_latin (s : u16x8) -> bool { let above_ascii = u16x8 :: splat (0x80) ; all_mask16x8 (s . simd_lt (above_ascii)) } # [inline (always)] pub fn simd_is_latin1 (s : u16x8) -> bool { let highest_latin1 = u16x8 :: splat (0xFF) ; ! any_mask16x8 (s . simd_gt (highest_latin1)) } } }
    };
}

macro_37!();