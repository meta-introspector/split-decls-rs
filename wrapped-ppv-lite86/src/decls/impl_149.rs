macro_rules! deps {
    () => {
        NoS4!();
        Vec4!();
        MultiLane!();
    };
}

macro_rules! impl_149 {
    () => {
        deps!();
        impl < S3 , NI > Vec4 < u32 > for u32x4_sse2 < S3 , NoS4 , NI > where Self : MultiLane < [u32 ; 4] > , { # [inline (always)] fn extract (self , i : u32) -> u32 { self . to_lanes () [i as usize] } # [inline (always)] fn insert (self , v : u32 , i : u32) -> Self { Self :: new (unsafe { match i { 0 => { let x = _mm_andnot_si128 (_mm_cvtsi32_si128 (- 1) , self . x) ; _mm_or_si128 (x , _mm_cvtsi32_si128 (v as i32)) } 1 => { let mut x = _mm_shuffle_epi32 (self . x , 0b0111_1000) ; x = _mm_slli_si128 (x , 4) ; x = _mm_or_si128 (x , _mm_cvtsi32_si128 (v as i32)) ; _mm_shuffle_epi32 (x , 0b1110_0001) } 2 => { let mut x = _mm_shuffle_epi32 (self . x , 0b1011_0100) ; x = _mm_slli_si128 (x , 4) ; x = _mm_or_si128 (x , _mm_cvtsi32_si128 (v as i32)) ; _mm_shuffle_epi32 (x , 0b1100_1001) } 3 => { let mut x = _mm_slli_si128 (self . x , 4) ; x = _mm_or_si128 (x , _mm_cvtsi32_si128 (v as i32)) ; _mm_shuffle_epi32 (x , 0b0011_1001) } _ => unreachable ! () , } }) } }
    };
}

impl_149!()