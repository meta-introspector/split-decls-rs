macro_rules! simd_unpack {
    () => {
        # [inline (always)] pub fn simd_unpack (s : u8x16) -> (u16x8 , u16x8) { let first : u8x16 = simd_swizzle ! (s , u8x16 :: splat (0) , [0 , 16 , 1 , 17 , 2 , 18 , 3 , 19 , 4 , 20 , 5 , 21 , 6 , 22 , 7 , 23]) ; let second : u8x16 = simd_swizzle ! (s , u8x16 :: splat (0) , [8 , 24 , 9 , 25 , 10 , 26 , 11 , 27 , 12 , 28 , 13 , 29 , 14 , 30 , 15 , 31]) ; (u16x8 :: from_ne_bytes (first) , u16x8 :: from_ne_bytes (second)) }
    };
}

simd_unpack!()