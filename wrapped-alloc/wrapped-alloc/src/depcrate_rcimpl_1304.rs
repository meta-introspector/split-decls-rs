// Generated macro for impl_1304 (impl)
macro_rules! Depcrate_rcimpl_1304 {
() => {
// Module: crate::rc
// Provides: {"impl_1304"}
// Dependencies: {}
# [unstable (feature = "unique_rc_arc" , issue = "112566")] impl < T : ? Sized , A : Allocator > borrow :: Borrow < T > for UniqueRc < T , A > { fn borrow (& self) -> & T { & * * self } }
};
}
