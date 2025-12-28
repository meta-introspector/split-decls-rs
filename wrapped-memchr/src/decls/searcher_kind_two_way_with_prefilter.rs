macro_rules! deps {
    () => {
        PrefilterState!();
        Searcher!();
        TwoWayWithPrefilter!();
        Pre!();
    };
}

macro_rules! searcher_kind_two_way_with_prefilter {
    () => {
        deps!();
        # [doc = " Reads from the `two_way_with_prefilter` field of `SearcherKind` to handle"] # [doc = " the case of searching for an arbitrary needle with prefilter acceleration."] # [doc = " Works on all platforms."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `searcher.kind.two_way_with_prefilter` union"] # [doc = " field is set."] unsafe fn searcher_kind_two_way_with_prefilter (searcher : & Searcher , prestate : & mut PrefilterState , haystack : & [u8] , needle : & [u8] ,) -> Option < usize > { if rabinkarp :: is_fast (haystack , needle) { searcher . rabinkarp . find (haystack , needle) } else { let TwoWayWithPrefilter { ref finder , ref prestrat } = searcher . kind . two_way_with_prefilter ; let pre = Pre { prestate , prestrat } ; finder . find_with_prefilter (Some (pre) , haystack , needle) } }
    };
}

searcher_kind_two_way_with_prefilter!();