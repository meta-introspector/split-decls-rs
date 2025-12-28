macro_rules! macro_34 {
    () => {
        cfg_if ! { if # [cfg (target_feature = "sse2")] { # [inline (always)] pub fn mask_ascii (s : u8x16) -> i32 { unsafe { _mm_movemask_epi8 (s . into ()) } } } else { } }
    };
}

macro_34!();