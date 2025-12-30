// Generated macro for impl_50 (impl)
macro_rules! Depcrate_streamimpl_50 {
() => {
// Module: crate::stream
// Provides: {"impl_50"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for MaybeHttpsStream < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match * self { Self :: Http (..) => f . pad ("Http(..)") , Self :: Https (..) => f . pad ("Https(..)") , } } }
};
}
