// Generated macro for SearcherKindFn (type)
macro_rules! Depcrate_memmem_searcherSearcherKindFn {
() => {
// Module: crate::memmem::searcher
// Provides: {"SearcherKindFn"}
// Dependencies: {}
# [doc = " The type of a substring search function."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " When using a function of this type, callers must ensure that the correct"] # [doc = " function is paired with the value populated in `SearcherKind` union."] type SearcherKindFn = unsafe fn (searcher : & Searcher , prestate : & mut PrefilterState , haystack : & [u8] , needle : & [u8] ,) -> Option < usize > ;
};
}
