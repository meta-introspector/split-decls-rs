macro_rules! deps {
    () => {
        Vec2!();
        NoS4!();
    };
}

macro_rules! impl_156 {
    () => {
        deps!();
        impl < S3 , NI > Vec2 < u64 > for u64x2_sse2 < S3 , NoS4 , NI > { # [inline (always)] fn extract (self , i : u32) -> u64 { unsafe { match i { 0 => _mm_cvtsi128_si64 (self . x) as u64 , 1 => _mm_cvtsi128_si64 (_mm_shuffle_epi32 (self . x , 0b11101110)) as u64 , _ => unreachable ! () , } } } # [inline (always)] fn insert (self , x : u64 , i : u32) -> Self { Self :: new (unsafe { match i { 0 => _mm_or_si128 (_mm_andnot_si128 (_mm_cvtsi64_si128 (- 1) , self . x) , _mm_cvtsi64_si128 (x as i64) ,) , 1 => _mm_or_si128 (_mm_move_epi64 (self . x) , _mm_slli_si128 (_mm_cvtsi64_si128 (x as i64) , 8) ,) , _ => unreachable ! () , } }) } }
    };
}

impl_156!()