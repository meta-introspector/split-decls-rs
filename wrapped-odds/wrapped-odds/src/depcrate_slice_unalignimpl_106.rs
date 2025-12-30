// Generated macro for impl_106 (impl)
macro_rules! Depcrate_slice_unalignimpl_106 {
() => {
// Module: crate::slice::unalign
// Provides: {"impl_106"}
// Dependencies: {}
impl < 'a , T > Iterator for UnalignedIter < 'a , T > where T : Copy , { type Item = T ; fn next (& mut self) -> Option < Self :: Item > { if self . ptr != self . end { unsafe { let elt = Some (ptr :: read_unaligned (self . ptr as * const T)) ; self . ptr = self . ptr . offset (size_of :: < T > () as isize) ; elt } } else { None } } }
};
}
