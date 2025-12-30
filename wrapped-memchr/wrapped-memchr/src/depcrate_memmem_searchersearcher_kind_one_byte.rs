// Generated macro for searcher_kind_one_byte (function)
macro_rules! Depcrate_memmem_searchersearcher_kind_one_byte {
() => {
// Module: crate::memmem::searcher
// Provides: {"searcher_kind_one_byte"}
// Dependencies: {}
# [doc = " Reads from the `one_byte` field of `SearcherKind` to handle the case of"] # [doc = " searching for a single byte needle. Works on all platforms."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " Callers must ensure that the `searcher.kind.one_byte` union field is set."] unsafe fn searcher_kind_one_byte (searcher : & Searcher , _prestate : & mut PrefilterState , haystack : & [u8] , _needle : & [u8] ,) -> Option < usize > { let needle = searcher . kind . one_byte ; crate :: memchr (needle , haystack) }
};
}
