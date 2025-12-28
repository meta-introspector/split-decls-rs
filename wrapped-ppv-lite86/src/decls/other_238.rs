macro_rules! other_238 {
    () => {
        # [allow (non_camel_case_types)] # [derive (Copy , Clone)] pub union vec512_storage { u32x16 : [u32 ; 16] , u64x8 : [u64 ; 8] , u128x4 : [u128 ; 4] , sse2 : [vec128_storage ; 4] , avx : [vec256_storage ; 2] , }
    };
}

other_238!();