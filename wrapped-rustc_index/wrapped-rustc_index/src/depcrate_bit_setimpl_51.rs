// Generated macro for impl_51 (impl)
macro_rules! Depcrate_bit_setimpl_51 {
() => {
// Module: crate::bit_set
// Provides: {"impl_51"}
// Dependencies: {}
impl < T > MixedBitSet < T > { pub fn domain_size (& self) -> usize { match self { MixedBitSet :: Small (set) => set . domain_size () , MixedBitSet :: Large (set) => set . domain_size () , } } }
};
}
