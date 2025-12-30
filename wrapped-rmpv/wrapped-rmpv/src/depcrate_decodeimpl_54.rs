// Generated macro for impl_54 (impl)
macro_rules! Depcrate_decodeimpl_54 {
() => {
// Module: crate::decode
// Provides: {"impl_54"}
// Dependencies: {}
impl Display for Error { # [cold] fn fmt (& self , fmt : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { match * self { Self :: InvalidMarkerRead (ref err) => { write ! (fmt , "I/O error while reading marker byte: {err}") } Self :: InvalidDataRead (ref err) => { write ! (fmt , "I/O error while reading non-marker bytes: {err}") } Self :: DepthLimitExceeded => { write ! (fmt , "depth limit exceeded") } } } }
};
}
