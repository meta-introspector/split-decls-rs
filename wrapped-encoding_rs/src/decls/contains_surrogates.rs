macro_rules! contains_surrogates {
    () => {
        # [inline (always)] pub fn contains_surrogates (s : u16x8) -> bool { let mask = u16x8 :: splat (0xF800) ; let surrogate_bits = u16x8 :: splat (0xD800) ; any_mask16x8 ((s & mask) . simd_eq (surrogate_bits)) }
    };
}

contains_surrogates!()