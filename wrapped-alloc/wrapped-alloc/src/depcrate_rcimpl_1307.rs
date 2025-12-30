// Generated macro for impl_1307 (impl)
macro_rules! Depcrate_rcimpl_1307 {
() => {
// Module: crate::rc
// Provides: {"impl_1307"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > AsMut < T > for UniqueRc < T , A > { fn as_mut (& mut self) -> & mut T { & mut * * self } }
};
}
