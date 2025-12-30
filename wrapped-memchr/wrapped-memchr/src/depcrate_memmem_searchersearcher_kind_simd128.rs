// Generated macro for searcher_kind_simd128 (function)
macro_rules! Depcrate_memmem_searchersearcher_kind_simd128 {
() => {
// Module: crate::memmem::searcher
// Provides: {"searcher_kind_simd128"}
// Dependencies: {}
# [doc = " Reads from the `simd128` field of `SearcherKind` to execute the wasm32"] # [doc = " simd128 vectorized substring search implementation."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `searcher.kind.simd128` union field is set."] # [cfg (all (target_arch = "wasm32" , target_feature = "simd128"))] unsafe fn searcher_kind_simd128 (searcher : & Searcher , _prestate : & mut PrefilterState , haystack : & [u8] , needle : & [u8] ,) -> Option < usize > { let finder = & searcher . kind . simd128 ; if haystack . len () < finder . min_haystack_len () { searcher . rabinkarp . find (haystack , needle) } else { finder . find (haystack , needle) } }
};
}
