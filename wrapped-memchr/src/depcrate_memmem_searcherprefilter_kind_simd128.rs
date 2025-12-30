// Generated macro for prefilter_kind_simd128 (function)
macro_rules! Depcrate_memmem_searcherprefilter_kind_simd128 {
() => {
// Module: crate::memmem::searcher
// Provides: {"prefilter_kind_simd128"}
// Dependencies: {}
# [doc = " Reads from the `simd128` field of `PrefilterKind` to execute the wasm32"] # [doc = " simd128 prefilter."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `strat.kind.simd128` union field is set."] # [cfg (all (target_arch = "wasm32" , target_feature = "simd128"))] unsafe fn prefilter_kind_simd128 (strat : & Prefilter , haystack : & [u8] ,) -> Option < usize > { let finder = & strat . kind . simd128 ; if haystack . len () < finder . min_haystack_len () { strat . find_simple (haystack) } else { finder . find_prefilter (haystack) } }
};
}
