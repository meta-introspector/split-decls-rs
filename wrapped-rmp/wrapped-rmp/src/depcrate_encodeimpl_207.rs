// Generated macro for impl_207 (impl)
macro_rules! Depcrate_encodeimpl_207 {
() => {
// Module: crate::encode
// Provides: {"impl_207"}
// Dependencies: {}
# [cfg (feature = "std")] impl < E : RmpWriteErr > error :: Error for ValueWriteError < E > { # [cold] fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { Self :: InvalidMarkerWrite (ref err) | Self :: InvalidDataWrite (ref err) => Some (err) , } } }
};
}
