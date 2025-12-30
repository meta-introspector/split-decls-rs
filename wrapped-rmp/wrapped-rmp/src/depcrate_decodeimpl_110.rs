// Generated macro for impl_110 (impl)
macro_rules! Depcrate_decodeimpl_110 {
() => {
// Module: crate::decode
// Provides: {"impl_110"}
// Dependencies: {}
impl < E : RmpReadErr > Display for NumValueReadError < E > { fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str (match * self { Self :: InvalidMarkerRead (..) => "failed to read MessagePack marker" , Self :: InvalidDataRead (..) => "failed to read MessagePack data" , Self :: TypeMismatch (..) => "the type decoded isn't match with the expected one" , Self :: OutOfRange => "out of range integral type conversion attempted" , }) } }
};
}
