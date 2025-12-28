macro_rules! deps {
    () => {
        PrefilterState!();
        Searcher!();
    };
}

macro_rules! searcher_kind_avx2 {
    () => {
        deps!();
        # [doc = " Reads from the `avx2` field of `SearcherKind` to execute the x86_64 AVX2"] # [doc = " vectorized substring search implementation."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `searcher.kind.avx2` union field is set."] # [cfg (all (target_arch = "x86_64" , target_feature = "sse2"))] unsafe fn searcher_kind_avx2 (searcher : & Searcher , _prestate : & mut PrefilterState , haystack : & [u8] , needle : & [u8] ,) -> Option < usize > { let finder = & searcher . kind . avx2 ; if haystack . len () < finder . min_haystack_len () { searcher . rabinkarp . find (haystack , needle) } else { finder . find (haystack , needle) } }
    };
}

searcher_kind_avx2!();