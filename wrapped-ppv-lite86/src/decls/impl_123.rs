macro_rules! deps {
    () => {
        YesS4!();
        MultiLane!();
    };
}

macro_rules! impl_123 {
    () => {
        deps!();
        impl < S3 , NI > MultiLane < [u32 ; 4] > for u32x4_sse2 < S3 , YesS4 , NI > { # [inline (always)] fn to_lanes (self) -> [u32 ; 4] { unsafe { let x = _mm_cvtsi128_si64 (self . x) as u64 ; let y = _mm_extract_epi64 (self . x , 1) as u64 ; [x as u32 , (x >> 32) as u32 , y as u32 , (y >> 32) as u32] } } # [inline (always)] fn from_lanes (xs : [u32 ; 4]) -> Self { unsafe { let mut x = _mm_cvtsi64_si128 ((xs [0] as u64 | ((xs [1] as u64) << 32)) as i64) ; x = _mm_insert_epi64 (x , (xs [2] as u64 | ((xs [3] as u64) << 32)) as i64 , 1) ; Self :: new (x) } } }
    };
}

impl_123!()