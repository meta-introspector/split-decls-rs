// Generated macro for impl_235 (impl)
macro_rules! Depcrate_module_lattice_encodeimpl_235 {
() => {
// Module: crate::module_lattice::encode
// Provides: {"impl_235"}
// Dependencies: {}
impl < F : Field , D : EncodingSize > Encode < D > for NttPolynomial < F > { type EncodedSize = D :: EncodedPolynomialSize ; fn encode (& self) -> Array < u8 , Self :: EncodedSize > { byte_encode :: < F , D > (& self . 0) } fn decode (enc : & Array < u8 , Self :: EncodedSize >) -> Self { Self (byte_decode :: < F , D > (enc)) } }
};
}
