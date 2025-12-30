// Generated macro for impl_32 (impl)
macro_rules! Depcrate_private_key_infoimpl_32 {
() => {
// Module: crate::private_key_info
// Provides: {"impl_32"}
// Dependencies: {}
impl < 'a , Params , Key , PubKey > EncodeValue for PrivateKeyInfo < Params , Key , PubKey > where Params : der :: Choice < 'a , Error = der :: Error > + Encode , Key : EncodeValue + FixedTag , PubKey : BitStringLike , { fn value_len (& self) -> der :: Result < Length > { self . version () . encoded_len () ? + self . algorithm . encoded_len () ? + self . private_key . encoded_len () ? + self . public_key_bit_string () . encoded_len () ? } fn encode_value (& self , writer : & mut impl Writer) -> der :: Result < () > { self . version () . encode (writer) ? ; self . algorithm . encode (writer) ? ; self . private_key . encode (writer) ? ; self . public_key_bit_string () . encode (writer) ? ; Ok (()) } }
};
}
