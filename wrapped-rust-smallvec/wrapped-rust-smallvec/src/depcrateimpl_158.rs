// Generated macro for impl_158 (impl)
macro_rules! Depcrateimpl_158 {
() => {
// Module: crate
// Provides: {"impl_158"}
// Dependencies: {}
impl < T : Clone , const M : usize , const N : usize > From < & [T ; M] > for SmallVec < T , N > { # [inline] fn from (slice : & [T ; M]) -> Self { Self :: from (slice as & [T]) } }
};
}
