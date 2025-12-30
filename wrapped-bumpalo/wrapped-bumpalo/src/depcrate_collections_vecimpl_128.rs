// Generated macro for impl_128 (impl)
macro_rules! Depcrate_collections_vecimpl_128 {
() => {
// Module: crate::collections::vec
// Provides: {"impl_128"}
// Dependencies: {}
impl < 'bump , T > Drop for Vec < 'bump , T > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (ptr :: slice_from_raw_parts_mut (self . as_mut_ptr () , self . len)) } } }
};
}
