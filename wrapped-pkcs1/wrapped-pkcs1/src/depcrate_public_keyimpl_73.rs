// Generated macro for impl_73 (impl)
macro_rules! Depcrate_public_keyimpl_73 {
() => {
// Module: crate::public_key
// Provides: {"impl_73"}
// Dependencies: {}
impl EncodeValue for RsaPublicKey < '_ > { fn value_len (& self) -> der :: Result < Length > { self . modulus . encoded_len () ? + self . public_exponent . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { self . modulus . encode (writer) ? ; self . public_exponent . encode (writer) ? ; Ok (()) } }
};
}
