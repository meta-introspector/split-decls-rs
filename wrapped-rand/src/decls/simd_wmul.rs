macro_rules! deps {
    () => {
        WideningMultiply!();
    };
}

macro_rules! simd_wmul {
    () => {
        deps!();
        # [cfg (feature = "simd_support")] mod simd_wmul { use super :: * ; # [cfg (target_arch = "x86")] use core :: arch :: x86 :: * ; # [cfg (target_arch = "x86_64")] use core :: arch :: x86_64 :: * ; wmul_impl ! { (u8x4 , u16x4) , (u8x8 , u16x8) , (u8x16 , u16x16) , (u8x32 , u16x32) , (u8x64 , Simd < u16 , 64 >) ,, 8 } wmul_impl ! { (u16x2 , u32x2) ,, 16 } wmul_impl ! { (u16x4 , u32x4) ,, 16 } # [cfg (not (target_feature = "sse2"))] wmul_impl ! { (u16x8 , u32x8) ,, 16 } # [cfg (not (target_feature = "avx2"))] wmul_impl ! { (u16x16 , u32x16) ,, 16 } # [cfg (not (target_feature = "avx512bw"))] wmul_impl ! { (u16x32 , Simd < u32 , 32 >) ,, 16 } # [allow (unused_macros)] macro_rules ! wmul_impl_16 { ($ ty : ident , $ mulhi : ident , $ mullo : ident) => { impl WideningMultiply for $ ty { type Output = ($ ty , $ ty) ; # [inline (always)] fn wmul (self , x : $ ty) -> Self :: Output { let hi = unsafe { $ mulhi (self . into () , x . into ()) } . into () ; let lo = unsafe { $ mullo (self . into () , x . into ()) } . into () ; (hi , lo) } } } ; } # [cfg (target_feature = "sse2")] wmul_impl_16 ! { u16x8 , _mm_mulhi_epu16 , _mm_mullo_epi16 } # [cfg (target_feature = "avx2")] wmul_impl_16 ! { u16x16 , _mm256_mulhi_epu16 , _mm256_mullo_epi16 } # [cfg (target_feature = "avx512bw")] wmul_impl_16 ! { u16x32 , _mm512_mulhi_epu16 , _mm512_mullo_epi16 } wmul_impl ! { (u32x2 , u64x2) , (u32x4 , u64x4) , (u32x8 , u64x8) , (u32x16 , Simd < u64 , 16 >) ,, 32 } wmul_impl_large ! { (u64x2 , u64x4 , u64x8 ,) u64 , 32 } }
    };
}

simd_wmul!()