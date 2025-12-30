// Generated macro for impl_109 (impl)
macro_rules! Depcrate_decodeimpl_109 {
() => {
// Module: crate::decode
// Provides: {"impl_109"}
// Dependencies: {}
# [cfg (feature = "std")] impl error :: Error for NumValueReadError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { Self :: InvalidMarkerRead (ref err) | Self :: InvalidDataRead (ref err) => Some (err) , Self :: TypeMismatch (..) | Self :: OutOfRange => None , } } }
};
}
