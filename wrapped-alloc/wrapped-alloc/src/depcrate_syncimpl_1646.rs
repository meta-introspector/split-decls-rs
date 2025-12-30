// Generated macro for impl_1646 (impl)
macro_rules! Depcrate_syncimpl_1646 {
() => {
// Module: crate::sync
// Provides: {"impl_1646"}
// Dependencies: {}
# [cfg (not (no_global_oom_handling))] impl < T : ? Sized , A : Allocator > Drop for UniqueArcUninit < T , A > { fn drop (& mut self) { unsafe { self . alloc . take () . unwrap () . deallocate (self . ptr . cast () , arcinner_layout_for_value_layout (self . layout_for_value) ,) ; } } }
};
}
