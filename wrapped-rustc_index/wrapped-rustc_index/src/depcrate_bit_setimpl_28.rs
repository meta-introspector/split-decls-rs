// Generated macro for impl_28 (impl)
macro_rules! Depcrate_bit_setimpl_28 {
() => {
// Module: crate::bit_set
// Provides: {"impl_28"}
// Dependencies: {}
impl < T : Idx > fmt :: Debug for DenseBitSet < T > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { w . debug_list () . entries (self . iter ()) . finish () } }
};
}
