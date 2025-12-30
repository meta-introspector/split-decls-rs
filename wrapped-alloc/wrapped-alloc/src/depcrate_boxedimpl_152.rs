// Generated macro for impl_152 (impl)
macro_rules! Depcrate_boxedimpl_152 {
() => {
// Module: crate::boxed
// Provides: {"impl_152"}
// Dependencies: {}
# [stable (feature = "box_borrow" , since = "1.1.0")] impl < T : ? Sized , A : Allocator > BorrowMut < T > for Box < T , A > { fn borrow_mut (& mut self) -> & mut T { & mut * * self } }
};
}
