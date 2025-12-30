// Generated macro for impl_1360 (impl)
macro_rules! Depcrate_sliceimpl_1360 {
() => {
// Module: crate::slice
// Provides: {"impl_1360"}
// Dependencies: {}
# [stable (feature = "rust1" , since = "1.0.0")] impl < T , A : Allocator > BorrowMut < [T] > for Vec < T , A > { fn borrow_mut (& mut self) -> & mut [T] { & mut self [..] } }
};
}
