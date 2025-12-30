// Generated macro for impl_1657 (impl)
macro_rules! Depcrate_syncimpl_1657 {
() => {
// Module: crate::sync
// Provides: {"impl_1657"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > borrow :: BorrowMut < T > for UniqueArc < T , A > { fn borrow_mut (& mut self) -> & mut T { & mut * * self } }
};
}
