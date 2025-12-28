macro_rules! deps {
    () => {
        PrefilterState!();
        Searcher!();
    };
}

macro_rules! searcher_kind_two_way {
    () => {
        deps!();
        # [doc = " Reads from the `two_way` field of `SearcherKind` to handle the case of"] # [doc = " searching for an arbitrary needle without prefilter acceleration. Works on"] # [doc = " all platforms."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `searcher.kind.two_way` union field is set."] unsafe fn searcher_kind_two_way (searcher : & Searcher , _prestate : & mut PrefilterState , haystack : & [u8] , needle : & [u8] ,) -> Option < usize > { if rabinkarp :: is_fast (haystack , needle) { searcher . rabinkarp . find (haystack , needle) } else { searcher . kind . two_way . find (haystack , needle) } }
    };
}

searcher_kind_two_way!()