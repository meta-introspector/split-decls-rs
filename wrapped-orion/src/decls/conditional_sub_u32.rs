macro_rules! conditional_sub_u32 {
    () => {
        fn conditional_sub_u32 (a : u32) -> u32 { let t : u32 = a . overflowing_sub (KYBER_Q) . 0 ; let mask : u32 = 0u32 . overflowing_sub (t >> 31) . 0 ; (t & ! mask) | (a & mask) }
    };
}

conditional_sub_u32!()