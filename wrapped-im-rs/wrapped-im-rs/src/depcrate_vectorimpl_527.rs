// Generated macro for impl_527 (impl)
macro_rules! Depcrate_vectorimpl_527 {
() => {
// Module: crate::vector
// Provides: {"impl_527"}
// Dependencies: {}
impl < A : Clone + Hash > Hash for Vector < A > { fn hash < H : Hasher > (& self , state : & mut H) { for i in self { i . hash (state) } } }
};
}
