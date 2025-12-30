// Generated macro for impl_1656 (impl)
macro_rules! Depcrate_syncimpl_1656 {
() => {
// Module: crate::sync
// Provides: {"impl_1656"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > borrow :: Borrow < T > for UniqueArc < T , A > { fn borrow (& self) -> & T { & * * self } }
};
}
