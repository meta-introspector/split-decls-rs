// Generated macro for impl_1963 (impl)
macro_rules! Depcrate_vecimpl_1963 {
() => {
// Module: crate::vec
// Provides: {"impl_1963"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] unsafe impl < # [may_dangle] T , A : Allocator > Drop for Vec < T , A > { fn drop (& mut self) { unsafe { ptr :: drop_in_place (ptr :: slice_from_raw_parts_mut (self . as_mut_ptr () , self . len)) } } }
};
}
