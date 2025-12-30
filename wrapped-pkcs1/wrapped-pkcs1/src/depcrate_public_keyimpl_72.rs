// Generated macro for impl_72 (impl)
macro_rules! Depcrate_public_keyimpl_72 {
() => {
// Module: crate::public_key
// Provides: {"impl_72"}
// Dependencies: {}
impl < 'a > DecodeValue < 'a > for RsaPublicKey < 'a > { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : Header) -> der :: Result < Self > { Ok (Self { modulus : reader . decode () ? , public_exponent : reader . decode () ? , }) } }
};
}
