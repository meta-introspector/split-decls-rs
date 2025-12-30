// Generated macro for impl_34 (impl)
macro_rules! Depcrate_decode_strimpl_34 {
() => {
// Module: crate::decode::str
// Provides: {"impl_34"}
// Dependencies: {}
# [cfg (feature = "std")] impl < E : RmpReadErr > error :: Error for DecodeStringError < '_ , E > { # [cold] fn source (& self) -> Option < & (dyn error :: Error + 'static) > { match * self { DecodeStringError :: InvalidMarkerRead (ref err) | DecodeStringError :: InvalidDataRead (ref err) => Some (err) , DecodeStringError :: TypeMismatch (..) | DecodeStringError :: BufferSizeTooSmall (..) => None , DecodeStringError :: InvalidUtf8 (_ , ref err) => Some (err) , } } }
};
}
