// Generated macro for impl_242 (impl)
macro_rules! Depcrateimpl_242 {
() => {
// Module: crate
// Provides: {"impl_242"}
// Dependencies: {}
impl < T , N : ArrayLength > Deref for GenericArray < T , N > { type Target = [T] ; # [inline (always)] fn deref (& self) -> & [T] { GenericArray :: as_slice (self) } }
};
}
