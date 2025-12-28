macro_rules! other_232 {
    () => {
        # [allow (non_camel_case_types)] # [derive (Copy , Clone)] pub union vec256_storage { u32x8 : [u32 ; 8] , u64x4 : [u64 ; 4] , u128x2 : [u128 ; 2] , sse2 : [vec128_storage ; 2] , avx : __m256i , }
    };
}

other_232!()