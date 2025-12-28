macro_rules! deps {
    () => {
        Searcher!();
        PrefilterState!();
    };
}

macro_rules! searcher_kind_empty {
    () => {
        deps!();
        # [doc = " Reads from the `empty` field of `SearcherKind` to handle the case of"] # [doc = " searching for the empty needle. Works on all platforms."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `searcher.kind.empty` union field is set."] unsafe fn searcher_kind_empty (_searcher : & Searcher , _prestate : & mut PrefilterState , _haystack : & [u8] , _needle : & [u8] ,) -> Option < usize > { Some (0) }
    };
}

searcher_kind_empty!()