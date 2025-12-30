// Generated macro for impl_52 (impl)
macro_rules! Depcrate_decodeimpl_52 {
() => {
// Module: crate::decode
// Provides: {"impl_52"}
// Dependencies: {}
impl Error { # [cold] # [must_use] pub fn kind (& self) -> ErrorKind { match * self { Self :: InvalidMarkerRead (ref err) => err . kind () , Self :: InvalidDataRead (ref err) => err . kind () , Self :: DepthLimitExceeded => ErrorKind :: Unsupported , } } }
};
}
