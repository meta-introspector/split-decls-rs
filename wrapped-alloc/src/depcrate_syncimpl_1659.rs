// Generated macro for impl_1659 (impl)
macro_rules! Depcrate_syncimpl_1659 {
() => {
// Module: crate::sync
// Provides: {"impl_1659"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > AsMut < T > for UniqueArc < T , A > { fn as_mut (& mut self) -> & mut T { & mut * * self } }
};
}
