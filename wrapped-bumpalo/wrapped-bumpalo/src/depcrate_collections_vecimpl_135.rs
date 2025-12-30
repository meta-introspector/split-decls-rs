// Generated macro for impl_135 (impl)
macro_rules! Depcrate_collections_vecimpl_135 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_135"}
// Dependencies: {}
impl < 'bump , T : 'bump > DoubleEndedIterator for IntoIter < 'bump , T > { # [inline] fn next_back (& mut self) -> Option < T > { unsafe { if self . end == self . ptr { None } else if mem :: size_of :: < T > () == 0 { self . end = arith_offset (self . end as * const i8 , - 1) as * mut T ; Some (mem :: zeroed ()) } else { self . end = self . end . offset (- 1) ; Some (ptr :: read (self . end)) } } } }
};
}
