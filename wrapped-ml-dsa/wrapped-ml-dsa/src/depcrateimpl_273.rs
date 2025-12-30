// Generated macro for impl_273 (impl)
macro_rules! Depcrateimpl_273 {
() => {
// Module: crate
// Provides: {"impl_273"}
// Dependencies: {}
impl < P : MlDsaParams > Signature < P > { # [doc = " Encode the signature in a fixed-size byte array."] pub fn encode (& self) -> EncodedSignature < P > { let c_tilde = self . c_tilde . clone () ; let z = P :: encode_z (& self . z) ; let h = self . h . bit_pack () ; P :: concat_sig (c_tilde , z , h) } # [doc = " Decode the signature from an appropriately sized byte array."] pub fn decode (enc : & EncodedSignature < P >) -> Option < Self > { let (c_tilde , z , h) = P :: split_sig (enc) ; let c_tilde = c_tilde . clone () ; let z = P :: decode_z (z) ; let h = Hint :: bit_unpack (h) ? ; if z . infinity_norm () >= P :: GAMMA1_MINUS_BETA { return None ; } Some (Self { c_tilde , z , h }) } }
};
}
