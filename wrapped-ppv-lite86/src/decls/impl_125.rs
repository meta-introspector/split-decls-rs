macro_rules! deps {
    () => {
        YesS4!();
        MultiLane!();
    };
}

macro_rules! impl_125 {
    () => {
        deps!();
        impl < S3 , NI > MultiLane < [u64 ; 2] > for u64x2_sse2 < S3 , YesS4 , NI > { # [inline (always)] fn to_lanes (self) -> [u64 ; 2] { unsafe { [_mm_cvtsi128_si64 (self . x) as u64 , _mm_extract_epi64 (self . x , 1) as u64 ,] } } # [inline (always)] fn from_lanes (xs : [u64 ; 2]) -> Self { unsafe { let mut x = _mm_cvtsi64_si128 (xs [0] as i64) ; x = _mm_insert_epi64 (x , xs [1] as i64 , 1) ; Self :: new (x) } } }
    };
}

impl_125!()