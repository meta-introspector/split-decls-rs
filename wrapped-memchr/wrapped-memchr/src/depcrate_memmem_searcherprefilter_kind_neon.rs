// Generated macro for prefilter_kind_neon (function)
macro_rules! Depcrate_memmem_searcherprefilter_kind_neon {
() => {
// Module: crate::memmem::searcher
// Provides: {"prefilter_kind_neon"}
// Dependencies: {}
# [doc = " Reads from the `neon` field of `PrefilterKind` to execute the aarch64 neon"] # [doc = " prefilter."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `strat.kind.neon` union field is set."] # [cfg (target_arch = "aarch64")] unsafe fn prefilter_kind_neon (strat : & Prefilter , haystack : & [u8] ,) -> Option < usize > { let finder = & strat . kind . neon ; if haystack . len () < finder . min_haystack_len () { strat . find_simple (haystack) } else { finder . find_prefilter (haystack) } }
};
}
