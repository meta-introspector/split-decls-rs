// Generated macro for impl_91 (impl)
macro_rules! Depcrate_rawimpl_91 {
() => {
// Module: crate::raw
// Provides: {"impl_91"}
// Dependencies: {}
# [cfg (not (feature = "nightly"))] impl < T , A : Allocator > Drop for RawTable < T , A > { # [cfg_attr (feature = "inline-more" , inline)] fn drop (& mut self) { unsafe { self . table . drop_inner_table :: < T , _ > (& self . alloc , Self :: TABLE_LAYOUT) ; } } }
};
}
