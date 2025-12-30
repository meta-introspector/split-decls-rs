// Generated macro for impl_1716 (impl)
macro_rules! Depcrate_vecimpl_1716 {
() => {
// Module: crate::vec
// Provides: {"impl_1716"}
// Dependencies: {}
impl < 'data , T : 'data > Drop for SliceDrain < 'data , T > { fn drop (& mut self) { let slice_ptr : * mut [T] = mem :: replace (& mut self . iter , [] . iter_mut ()) . into_slice () ; unsafe { ptr :: drop_in_place :: < [T] > (slice_ptr) } ; } }
};
}
