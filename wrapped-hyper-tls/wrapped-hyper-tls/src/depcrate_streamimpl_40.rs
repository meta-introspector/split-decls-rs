// Generated macro for impl_40 (impl)
macro_rules! Depcrate_streamimpl_40 {
() => {
// Module: crate::stream
// Provides: {"impl_40"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for MaybeHttpsStream < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self { MaybeHttpsStream :: Http (s) => f . debug_tuple ("Http") . field (s) . finish () , MaybeHttpsStream :: Https (s) => f . debug_tuple ("Https") . field (s) . finish () , } } }
};
}
