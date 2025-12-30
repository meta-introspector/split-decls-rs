// Generated macro for impl_560 (impl)
macro_rules! Depcrate_vectorimpl_560 {
() => {
// Module: crate::vector
// Provides: {"impl_560"}
// Dependencies: {}
impl < 'a , A : Clone > Chunks < 'a , A > { fn new (seq : & 'a Vector < A >) -> Self { Chunks { focus : seq . focus () , front_index : 0 , back_index : seq . len () , } } }
};
}
