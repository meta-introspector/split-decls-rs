// Generated macro for impl_118 (impl)
macro_rules! Depcrate_utils_wakers_array_no_stdimpl_118 {
() => {
// Module: crate::utils::wakers::array::no_std
// Provides: {"impl_118"}
// Dependencies: {}
impl < 'a , const N : usize > Deref for ReadinessArrayRef < 'a , N > { type Target = ReadinessArray < N > ; fn deref (& self) -> & Self :: Target { self . inner } }
};
}
