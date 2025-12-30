// Generated macro for impl_151 (impl)
macro_rules! Depcrate_read_utilimpl_151 {
() => {
// Module: crate::read::util
// Provides: {"impl_151"}
// Dependencies: {}
impl < T , const N : usize > ArrayLike for [T ; N] { type Item = T ; fn as_slice (storage : & Self :: Storage) -> & [MaybeUninit < T >] { storage } fn as_mut_slice (storage : & mut Self :: Storage) -> & mut [MaybeUninit < T >] { storage } }
};
}
