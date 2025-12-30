// Generated macro for impl_402 (impl)
macro_rules! Depcrate_memmem_searcherimpl_402 {
() => {
// Module: crate::memmem::searcher
// Provides: {"impl_402"}
// Dependencies: {}
impl < 'a > Pre < 'a > { # [doc = " Call this prefilter on the given haystack with the given needle."] # [inline] pub (crate) fn find (& mut self , haystack : & [u8]) -> Option < usize > { let result = self . prestrat . find (haystack) ; self . prestate . update (result . unwrap_or (haystack . len ())) ; result } # [doc = " Return true if and only if this prefilter should be used."] # [inline] pub (crate) fn is_effective (& mut self) -> bool { self . prestate . is_effective () } }
};
}
