// Generated macro for impl_421 (impl)
macro_rules! Depcrate_util_prefilterimpl_421 {
() => {
// Module: crate::util::prefilter
// Provides: {"impl_421"}
// Dependencies: {}
impl PrefilterI for Packed { fn find_in (& self , haystack : & [u8] , span : Span) -> Candidate { self . 0 . find_in (haystack , span) . map_or (Candidate :: None , Candidate :: Match) } }
};
}
