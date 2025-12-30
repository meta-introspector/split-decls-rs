// Generated macro for impl_1291 (impl)
macro_rules! Depcrate_rcimpl_1291 {
() => {
// Module: crate::rc
// Provides: {"impl_1291"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T : ? Sized , A : Allocator > borrow :: Borrow < T > for Rc < T , A > { fn borrow (& self) -> & T { & * * self } }
};
}
