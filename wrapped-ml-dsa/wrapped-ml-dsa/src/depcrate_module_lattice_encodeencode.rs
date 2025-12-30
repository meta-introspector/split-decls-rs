// Generated macro for Encode (trait)
macro_rules! Depcrate_module_lattice_encodeEncode {
() => {
// Module: crate::module_lattice::encode
// Provides: {"Encode"}
// Dependencies: {}
pub (crate) trait Encode < D : EncodingSize > { type EncodedSize : ArraySize ; fn encode (& self) -> Array < u8 , Self :: EncodedSize > ; fn decode (enc : & Array < u8 , Self :: EncodedSize >) -> Self ; }
};
}
