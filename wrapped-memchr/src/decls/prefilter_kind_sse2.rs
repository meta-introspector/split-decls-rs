macro_rules! deps {
    () => {
        Prefilter!();
    };
}

macro_rules! prefilter_kind_sse2 {
    () => {
        deps!();
        # [doc = " Reads from the `sse2` field of `PrefilterKind` to execute the x86_64 SSE2"] # [doc = " prefilter."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `strat.kind.sse2` union field is set."] # [cfg (all (target_arch = "x86_64" , target_feature = "sse2"))] unsafe fn prefilter_kind_sse2 (strat : & Prefilter , haystack : & [u8] ,) -> Option < usize > { let finder = & strat . kind . sse2 ; if haystack . len () < finder . min_haystack_len () { strat . find_simple (haystack) } else { finder . find_prefilter (haystack) } }
    };
}

prefilter_kind_sse2!()