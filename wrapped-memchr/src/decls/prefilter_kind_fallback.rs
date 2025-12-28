macro_rules! deps {
    () => {
        Prefilter!();
    };
}

macro_rules! prefilter_kind_fallback {
    () => {
        deps!();
        # [doc = " Reads from the `fallback` field of `PrefilterKind` to execute the fallback"] # [doc = " prefilter. Works on all platforms."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `strat.kind.fallback` union field is set."] unsafe fn prefilter_kind_fallback (strat : & Prefilter , haystack : & [u8] ,) -> Option < usize > { strat . kind . fallback . find_prefilter (haystack) }
    };
}

prefilter_kind_fallback!()