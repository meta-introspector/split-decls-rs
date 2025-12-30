// Generated macro for impl_1323 (impl)
macro_rules! Depcrate_rcimpl_1323 {
() => {
// Module: crate::rc
// Provides: {"impl_1323"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : ? Sized , A : Allocator > Drop for UniqueRcUninit < T , A > { fn drop (& mut self) { unsafe { self . alloc . take () . unwrap () . deallocate (self . ptr . cast () , rc_inner_layout_for_value_layout (self . layout_for_value) ,) ; } } }
};
}
