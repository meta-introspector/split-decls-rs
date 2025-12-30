// Generated macro for impl_153 (impl)
macro_rules! Depcrate_read_utilimpl_153 {
() => {
// Module: crate::read::util
// Provides: {"impl_153"}
// Dependencies: {}
# [cfg (feature = "read")] impl < T , const N : usize > ArrayLike for Box < [T ; N] > { type Item = T ; fn as_slice (storage : & Self :: Storage) -> & [MaybeUninit < T >] { & storage [..] } fn as_mut_slice (storage : & mut Self :: Storage) -> & mut [MaybeUninit < T >] { & mut storage [..] } }
};
}
