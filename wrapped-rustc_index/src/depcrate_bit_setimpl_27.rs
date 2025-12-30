// Generated macro for impl_27 (impl)
macro_rules! Depcrate_bit_setimpl_27 {
() => {
// Module: crate::bit_set
// Provides: {"impl_27"}
// Dependencies: {}
impl < T > Clone for DenseBitSet < T > { fn clone (& self) -> Self { DenseBitSet { domain_size : self . domain_size , words : self . words . clone () , marker : PhantomData , } } fn clone_from (& mut self , from : & Self) { self . domain_size = from . domain_size ; self . words . clone_from (& from . words) ; } }
};
}
