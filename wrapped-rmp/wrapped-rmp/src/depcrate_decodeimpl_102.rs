// Generated macro for impl_102 (impl)
macro_rules! Depcrate_decodeimpl_102 {
() => {
// Module: crate::decode
// Provides: {"impl_102"}
// Dependencies: {}
impl Display for ValueReadError { # [cold] fn fmt (& self , f : & mut Formatter < '_ >) -> Result < () , fmt :: Error > { f . write_str (match * self { Self :: InvalidMarkerRead (..) => "failed to read MessagePack marker" , Self :: InvalidDataRead (..) => "failed to read MessagePack data" , Self :: TypeMismatch (..) => "the type decoded isn't match with the expected one" , }) } }
};
}
