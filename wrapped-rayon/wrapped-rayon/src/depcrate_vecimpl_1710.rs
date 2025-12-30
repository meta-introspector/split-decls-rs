// Generated macro for impl_1710 (impl)
macro_rules! Depcrate_vecimpl_1710 {
() => {
// Module: crate::vec
// Provides: {"impl_1710"}
// Dependencies: {}
impl < 'data , T : 'data + Send > Drop for DrainProducer < 'data , T > { fn drop (& mut self) { let slice_ptr : * mut [T] = mem :: take :: < & 'data mut [T] > (& mut self . slice) ; unsafe { ptr :: drop_in_place :: < [T] > (slice_ptr) } ; } }
};
}
