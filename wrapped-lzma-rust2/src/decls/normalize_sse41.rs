macro_rules! normalize_sse41 {
    () => {
        # [doc = " Normalization implementation using SSE4.1 for 128-bit SIMD processing."] # [cfg (all (feature = "std" , feature = "optimization" , target_arch = "x86_64"))] # [target_feature (enable = "sse4.1")] unsafe fn normalize_sse41 (positions : & mut [i32] , norm_offset : i32) { use core :: arch :: x86_64 :: * ; let norm_v = _mm_set1_epi32 (norm_offset) ; let (prefix , chunks , suffix) = positions . align_to_mut :: < __m128i > () ; normalize_scalar (prefix , norm_offset) ; for chunk in chunks { let data = _mm_load_si128 (chunk as * mut _) ; let max_val = _mm_max_epi32 (data , norm_v) ; let result = _mm_sub_epi32 (max_val , norm_v) ; _mm_store_si128 (chunk as * mut _ , result) ; } normalize_scalar (suffix , norm_offset) ; }
    };
}

normalize_sse41!()