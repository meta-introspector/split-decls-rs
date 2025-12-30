// Generated macro for impl_36 (impl)
macro_rules! Depcrate_aeadimpl_36 {
() => {
// Module: crate::aead
// Provides: {"impl_36"}
// Dependencies: {}
impl < N : NonceSequence > Debug for SealingKey < N > { fn fmt (& self , f : & mut core :: fmt :: Formatter) -> Result < () , core :: fmt :: Error > { f . debug_struct ("SealingKey") . field ("algorithm" , & self . algorithm ()) . finish () } }
};
}
