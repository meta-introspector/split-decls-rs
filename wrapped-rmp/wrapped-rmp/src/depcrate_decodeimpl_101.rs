// Generated macro for impl_101 (impl)
macro_rules! Depcrate_decodeimpl_101 {
() => {
// Module: crate::decode
// Provides: {"impl_101"}
// Dependencies: {}
# [cfg (feature = "std")] impl error :: Error for ValueReadError { # [cold] fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { Self :: InvalidMarkerRead (ref err) | Self :: InvalidDataRead (ref err) => Some (err) , Self :: TypeMismatch (..) => None , } } }
};
}
