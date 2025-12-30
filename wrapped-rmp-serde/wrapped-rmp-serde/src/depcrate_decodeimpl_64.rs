// Generated macro for impl_64 (impl)
macro_rules! Depcrate_decodeimpl_64 {
() => {
// Module: crate::decode
// Provides: {"impl_64"}
// Dependencies: {}
impl error :: Error for Error { # [cold] fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { Self :: TypeMismatch (..) => None , Self :: InvalidMarkerRead (ref err) => Some (err) , Self :: InvalidDataRead (ref err) => Some (err) , Self :: LengthMismatch (..) => None , Self :: OutOfRange => None , Self :: Uncategorized (..) => None , Self :: Syntax (..) => None , Self :: Utf8Error (ref err) => Some (err) , Self :: DepthLimitExceeded => None , } } }
};
}
