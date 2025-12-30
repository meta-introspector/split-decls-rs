// Generated macro for prefilter_kind_fallback (function)
macro_rules! Depcrate_memmem_searcherprefilter_kind_fallback {
() => {
// Module: crate::memmem::searcher
// Provides: {"prefilter_kind_fallback"}
// Dependencies: {}
# [doc = " Reads from the `fallback` field of `PrefilterKind` to execute the fallback"] # [doc = " prefilter. Works on all platforms."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `strat.kind.fallback` union field is set."] unsafe fn prefilter_kind_fallback (strat : & Prefilter , haystack : & [u8] ,) -> Option < usize > { strat . kind . fallback . find_prefilter (haystack) }
};
}
