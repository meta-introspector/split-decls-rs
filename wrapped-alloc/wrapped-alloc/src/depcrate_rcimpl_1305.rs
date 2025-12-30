// Generated macro for impl_1305 (impl)
macro_rules! Depcrate_rcimpl_1305 {
() => {
// Module: crate::rc
// Provides: {"impl_1305"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > borrow :: BorrowMut < T > for UniqueRc < T , A > { fn borrow_mut (& mut self) -> & mut T { & mut * * self } }
};
}
