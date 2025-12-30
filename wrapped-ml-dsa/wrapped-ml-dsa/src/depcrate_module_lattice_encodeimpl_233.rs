// Generated macro for impl_233 (impl)
macro_rules! Depcrate_module_lattice_encodeimpl_233 {
() => {
// Module: crate::module_lattice::encode
// Provides: {"impl_233"}
// Dependencies: {}
impl < F : Field , D : EncodingSize > Encode < D > for Polynomial < F > { type EncodedSize = D :: EncodedPolynomialSize ; fn encode (& self) -> Array < u8 , Self :: EncodedSize > { byte_encode :: < F , D > (& self . 0) } fn decode (enc : & Array < u8 , Self :: EncodedSize >) -> Self { Self (byte_decode :: < F , D > (enc)) } }
};
}
