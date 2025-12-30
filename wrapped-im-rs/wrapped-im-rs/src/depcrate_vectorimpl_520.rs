// Generated macro for impl_520 (impl)
macro_rules! Depcrate_vectorimpl_520 {
() => {
// Module: crate::vector
// Provides: {"impl_520"}
// Dependencies: {}
impl < A : Clone + Debug > Debug for Vector < A > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , Error > { f . debug_list () . entries (self . iter ()) . finish () } }
};
}
