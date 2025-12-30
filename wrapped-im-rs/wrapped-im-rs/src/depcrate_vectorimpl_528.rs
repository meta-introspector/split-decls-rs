// Generated macro for impl_528 (impl)
macro_rules! Depcrate_vectorimpl_528 {
() => {
// Module: crate::vector
// Provides: {"impl_528"}
// Dependencies: {}
impl < A : Clone > Sum for Vector < A > { fn sum < I > (it : I) -> Self where I : Iterator < Item = Self > , { it . fold (Self :: new () , | a , b | a + b) } }
};
}
