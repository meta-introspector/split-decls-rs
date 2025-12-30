// Generated macro for impl_55 (impl)
macro_rules! Depcrate_bit_setimpl_55 {
() => {
// Module: crate::bit_set
// Provides: {"impl_55"}
// Dependencies: {}
impl < T : Idx > fmt :: Debug for MixedBitSet < T > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { MixedBitSet :: Small (set) => set . fmt (w) , MixedBitSet :: Large (set) => set . fmt (w) , } } }
};
}
