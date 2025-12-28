macro_rules! simd_byte_swap {
    () => {
        # [inline (always)] pub fn simd_byte_swap (s : u16x8) -> u16x8 { let left = s << 8 ; let right = s >> 8 ; left | right }
    };
}

simd_byte_swap!()