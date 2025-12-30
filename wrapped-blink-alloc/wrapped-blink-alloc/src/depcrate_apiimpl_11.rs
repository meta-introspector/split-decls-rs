// Generated macro for impl_11 (impl)
macro_rules! Depcrate_apiimpl_11 {
() => {
// Module: crate::api
// Provides: {"impl_11"}
// Dependencies: {}
unsafe impl < 'a , A > BlinkAllocator for & 'a mut A where & 'a mut A : Allocator , A : BlinkAllocator , { # [inline] fn reset (& mut self) { A :: reset (self) ; } }
};
}
