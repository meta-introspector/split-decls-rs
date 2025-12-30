// Generated macro for searcher_kind_sse2 (function)
macro_rules! Depcrate_memmem_searchersearcher_kind_sse2 {
() => {
// Module: crate::memmem::searcher
// Provides: {"searcher_kind_sse2"}
// Dependencies: {}
# [doc = " Reads from the `sse2` field of `SearcherKind` to execute the x86_64 SSE2"] # [doc = " vectorized substring search implementation."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `searcher.kind.sse2` union field is set."] # [cfg (all (target_arch = "x86_64" , target_feature = "sse2"))] unsafe fn searcher_kind_sse2 (searcher : & Searcher , _prestate : & mut PrefilterState , haystack : & [u8] , needle : & [u8] ,) -> Option < usize > { let finder = & searcher . kind . sse2 ; if haystack . len () < finder . min_haystack_len () { searcher . rabinkarp . find (haystack , needle) } else { finder . find (haystack , needle) } }
};
}
