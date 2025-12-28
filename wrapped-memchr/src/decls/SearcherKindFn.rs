macro_rules! deps {
    () => {
        Searcher!();
        PrefilterState!();
    };
}

macro_rules! SearcherKindFn {
    () => {
        deps!();
        # [doc = " The type of a substring search function."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " When using a function of this type, callers must ensure that the correct"] # [doc = " function is paired with the value populated in `SearcherKind` union."] type SearcherKindFn = unsafe fn (searcher : & Searcher , prestate : & mut PrefilterState , haystack : & [u8] , needle : & [u8] ,) -> Option < usize > ;
    };
}

SearcherKindFn!()