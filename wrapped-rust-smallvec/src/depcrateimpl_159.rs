// Generated macro for impl_159 (impl)
macro_rules! Depcrateimpl_159 {
() => {
// Module: crate
// Provides: {"impl_159"}
// Dependencies: {}
impl < T : Clone , const M : usize , const N : usize > From < & mut [T ; M] > for SmallVec < T , N > { # [inline] fn from (slice : & mut [T ; M]) -> Self { Self :: from (slice as & [T]) } }
};
}
