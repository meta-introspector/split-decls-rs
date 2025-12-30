// Generated macro for impl_155 (impl)
macro_rules! Depcrate_read_utilimpl_155 {
() => {
// Module: crate::read::util
// Provides: {"impl_155"}
// Dependencies: {}
# [cfg (feature = "read")] impl < T > ArrayLike for Vec < T > { type Item = T ; fn as_slice (storage : & Self :: Storage) -> & [MaybeUninit < T >] { storage } fn as_mut_slice (storage : & mut Self :: Storage) -> & mut [MaybeUninit < T >] { storage } }
};
}
