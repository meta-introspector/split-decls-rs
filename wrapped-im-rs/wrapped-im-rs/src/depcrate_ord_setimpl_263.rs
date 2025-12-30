// Generated macro for impl_263 (impl)
macro_rules! Depcrate_ord_setimpl_263 {
() => {
// Module: crate::ord::set
// Provides: {"impl_263"}
// Dependencies: {}
impl < A : Ord + Clone > Sum for OrdSet < A > { fn sum < I > (it : I) -> Self where I : Iterator < Item = Self > , { it . fold (Self :: new () , | a , b | a + b) } }
};
}
