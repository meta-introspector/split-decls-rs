// Generated macro for impl_51 (impl)
macro_rules! Depcrate_private_keyimpl_51 {
() => {
// Module: crate::private_key
// Provides: {"impl_51"}
// Dependencies: {}
impl EncodeValue for RsaPrivateKey < '_ > { fn value_len (& self) -> der :: Result < Length > { self . version () . encoded_len () ? + self . modulus . encoded_len () ? + self . public_exponent . encoded_len () ? + self . private_exponent . encoded_len () ? + self . prime1 . encoded_len () ? + self . prime2 . encoded_len () ? + self . exponent1 . encoded_len () ? + self . exponent2 . encoded_len () ? + self . coefficient . encoded_len () ? + self . other_prime_infos . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { self . version () . encode (writer) ? ; self . modulus . encode (writer) ? ; self . public_exponent . encode (writer) ? ; self . private_exponent . encode (writer) ? ; self . prime1 . encode (writer) ? ; self . prime2 . encode (writer) ? ; self . exponent1 . encode (writer) ? ; self . exponent2 . encode (writer) ? ; self . coefficient . encode (writer) ? ; self . other_prime_infos . encode (writer) ? ; Ok (()) } }
};
}
