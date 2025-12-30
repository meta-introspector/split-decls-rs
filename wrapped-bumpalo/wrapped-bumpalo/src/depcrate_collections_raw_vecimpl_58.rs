// Generated macro for impl_58 (impl)
macro_rules! Depcrate_collections_raw_vecimpl_58 {
() => {
// Module: crate::collections::raw_vec
// Provides: {"impl_58"}
// Dependencies: {}
impl < 'a , T > Drop for RawVec < 'a , T > { # [doc = " Frees the memory owned by the RawVec *without* trying to Drop its contents."] fn drop (& mut self) { unsafe { self . dealloc_buffer () ; } } }
};
}
