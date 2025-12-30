// Generated macro for impl_32 (impl)
macro_rules! Depcrate_aeadimpl_32 {
() => {
// Module: crate::aead
// Provides: {"impl_32"}
// Dependencies: {}
impl < N : NonceSequence > Debug for OpeningKey < N > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> Result < () , core :: fmt :: Error > { f . debug_struct ("OpeningKey") . field ("algorithm" , & self . algorithm ()) . finish () } }
};
}
