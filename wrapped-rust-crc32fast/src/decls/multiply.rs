macro_rules! multiply {
    () => {
        fn multiply (a : u32 , mut b : u32) -> u32 { let mut p = 0u32 ; for i in 0 .. 32 { p ^= b & ((a >> (31 - i)) & 1) . wrapping_neg () ; b = (b >> 1) ^ ((b & 1) . wrapping_neg () & POLY) ; } p }
    };
}

multiply!()