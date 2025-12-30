// Generated macro for impl_243 (impl)
macro_rules! Depcrateimpl_243 {
() => {
// Module: crate
// Provides: {"impl_243"}
// Dependencies: {}
impl < T , N : ArrayLength > DerefMut for GenericArray < T , N > { # [inline (always)] fn deref_mut (& mut self) -> & mut [T] { GenericArray :: as_mut_slice (self) } }
};
}
