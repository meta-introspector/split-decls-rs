// Generated macro for impl_53 (impl)
macro_rules! Depcrate_decodeimpl_53 {
() => {
// Module: crate::decode
// Provides: {"impl_53"}
// Dependencies: {}
impl error :: Error for Error { # [cold] fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { Self :: InvalidMarkerRead (ref err) => Some (err) , Self :: InvalidDataRead (ref err) => Some (err) , Self :: DepthLimitExceeded => None , } } }
};
}
