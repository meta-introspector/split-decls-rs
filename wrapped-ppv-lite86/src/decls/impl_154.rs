macro_rules! deps {
    () => {
        UnsafeFrom!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl < S3 , S4 , NI > UnsafeFrom < [u64 ; 2] > for u64x2_sse2 < S3 , S4 , NI > { # [inline (always)] unsafe fn unsafe_from (xs : [u64 ; 2]) -> Self { Self :: new (_mm_set_epi64x (xs [1] as i64 , xs [0] as i64)) } }
    };
}

impl_154!()