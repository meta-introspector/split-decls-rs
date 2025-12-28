macro_rules! deps {
    () => {
        Vec2!();
        YesS4!();
    };
}

macro_rules! impl_155 {
    () => {
        deps!();
        impl < S3 , NI > Vec2 < u64 > for u64x2_sse2 < S3 , YesS4 , NI > { # [inline (always)] fn extract (self , i : u32) -> u64 { unsafe { match i { 0 => _mm_cvtsi128_si64 (self . x) as u64 , 1 => _mm_extract_epi64 (self . x , 1) as u64 , _ => unreachable ! () , } } } # [inline (always)] fn insert (self , x : u64 , i : u32) -> Self { Self :: new (unsafe { match i { 0 => _mm_insert_epi64 (self . x , x as i64 , 0) , 1 => _mm_insert_epi64 (self . x , x as i64 , 1) , _ => unreachable ! () , } }) } }
    };
}

impl_155!()