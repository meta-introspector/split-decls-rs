// Generated macro for impl_47 (impl)
macro_rules! Depcrate_bit_setimpl_47 {
() => {
// Module: crate::bit_set
// Provides: {"impl_47"}
// Dependencies: {}
impl < T : Idx > fmt :: Debug for ChunkedBitSet < T > { fn fmt (& self , w : & mut fmt :: Formatter < '_ >) -> fmt :: Result { w . debug_list () . entries (self . iter ()) . finish () } }
};
}
