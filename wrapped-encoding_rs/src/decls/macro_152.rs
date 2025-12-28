macro_rules! macro_152 {
    () => {
        cfg_if ! { if # [cfg (feature = "simd-accel")] { use simd_funcs ::*; use core :: simd :: u16x8 ; use core :: simd :: cmp :: SimdPartialOrd ; # [inline (always)] fn shift_upper (unpacked : u16x8) -> u16x8 { let highest_ascii = u16x8 :: splat (0x7F) ; unpacked + unpacked . simd_gt (highest_ascii) . select (u16x8 :: splat (0xF700) , u16x8 :: splat (0)) } } else { } }
    };
}

macro_152!();