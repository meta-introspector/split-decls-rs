// Generated macro for impl_151 (impl)
macro_rules! Depcrate_boxedimpl_151 {
() => {
// Module: crate::boxed
// Provides: {"impl_151"}
// Dependencies: {}
# [stable (feature = "box_borrow" , since = "1.1.0")] impl < T : ? Sized , A : Allocator > Borrow < T > for Box < T , A > { fn borrow (& self) -> & T { & * * self } }
};
}
