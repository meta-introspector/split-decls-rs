// Generated macro for impl_34 (impl)
macro_rules! Depcrate_private_key_infoimpl_34 {
() => {
// Module: crate::private_key_info
// Provides: {"impl_34"}
// Dependencies: {}
impl < 'a , Params , Key , PubKey > TryFrom < & 'a [u8] > for PrivateKeyInfo < Params , Key , PubKey > where Params : der :: Choice < 'a , Error = der :: Error > + Encode , Key : DecodeValue < 'a , Error = der :: Error > + FixedTag + 'a , Key : EncodeValue , PubKey : DecodeValue < 'a , Error = der :: Error > + FixedTag + 'a , PubKey : BitStringLike , { type Error = Error ; fn try_from (bytes : & 'a [u8]) -> Result < Self > { Ok (Self :: from_der (bytes) ?) } }
};
}
