macro_rules! normalize_avx2 {
    () => {
        # [doc = " Normalization implementation using AVX2 for 256-bit SIMD processing."] # [cfg (all (feature = "std" , feature = "optimization" , target_arch = "x86_64"))] # [target_feature (enable = "avx2")] unsafe fn normalize_avx2 (positions : & mut [i32] , norm_offset : i32) { use core :: arch :: x86_64 :: * ; let norm_v = _mm256_set1_epi32 (norm_offset) ; let (prefix , chunks , suffix) = positions . align_to_mut :: < __m256i > () ; normalize_scalar (prefix , norm_offset) ; for chunk in chunks { let data = _mm256_load_si256 (chunk as * mut _) ; let max_val = _mm256_max_epi32 (data , norm_v) ; let result = _mm256_sub_epi32 (max_val , norm_v) ; _mm256_store_si256 (chunk as * mut _ , result) ; } normalize_scalar (suffix , norm_offset) ; }
    };
}

normalize_avx2!();