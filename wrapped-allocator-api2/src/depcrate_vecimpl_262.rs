// Generated macro for impl_262 (impl)
macro_rules! Depcrate_vecimpl_262 {
() => {
// Module: crate::vec
// Provides: {"impl_262"}
// Dependencies: {}
impl < T , A : Allocator > Drop for Vec < T , A > { # [inline (always)] fn drop (& mut self) { unsafe { ptr :: drop_in_place (ptr :: slice_from_raw_parts_mut (self . as_mut_ptr () , self . len)) } } }
};
}
