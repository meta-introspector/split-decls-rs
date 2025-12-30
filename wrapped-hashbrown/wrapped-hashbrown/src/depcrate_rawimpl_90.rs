// Generated macro for impl_90 (impl)
macro_rules! Depcrate_rawimpl_90 {
() => {
// Module: crate::raw
// Provides: {"impl_90"}
// Dependencies: {}
# [cfg (feature = "nightly")] unsafe impl < # [may_dangle] T , A : Allocator > Drop for RawTable < T , A > { # [cfg_attr (feature = "inline-more" , inline)] fn drop (& mut self) { unsafe { self . table . drop_inner_table :: < T , _ > (& self . alloc , Self :: TABLE_LAYOUT) ; } } }
};
}
