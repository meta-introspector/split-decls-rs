// Generated macro for impl_157 (impl)
macro_rules! Depcrateimpl_157 {
() => {
// Module: crate
// Provides: {"impl_157"}
// Dependencies: {}
impl < T : Clone , const N : usize > From < & mut [T] > for SmallVec < T , N > { # [inline] fn from (slice : & mut [T]) -> Self { Self :: from (slice as & [T]) } }
};
}
