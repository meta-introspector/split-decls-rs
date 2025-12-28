macro_rules! deps {
    () => {
        Searcher!();
        PrefilterState!();
    };
}

macro_rules! searcher_kind_neon {
    () => {
        deps!();
        # [doc = " Reads from the `neon` field of `SearcherKind` to execute the aarch64 neon"] # [doc = " vectorized substring search implementation."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `searcher.kind.neon` union field is set."] # [cfg (target_arch = "aarch64")] unsafe fn searcher_kind_neon (searcher : & Searcher , _prestate : & mut PrefilterState , haystack : & [u8] , needle : & [u8] ,) -> Option < usize > { let finder = & searcher . kind . neon ; if haystack . len () < finder . min_haystack_len () { searcher . rabinkarp . find (haystack , needle) } else { finder . find (haystack , needle) } }
    };
}

searcher_kind_neon!();