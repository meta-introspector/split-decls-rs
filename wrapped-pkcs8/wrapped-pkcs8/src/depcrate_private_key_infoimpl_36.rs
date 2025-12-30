// Generated macro for impl_36 (impl)
macro_rules! Depcrate_private_key_infoimpl_36 {
() => {
// Module: crate::private_key_info
// Provides: {"impl_36"}
// Dependencies: {}
# [cfg (feature = "alloc")] impl < 'a , Params , Key , PubKey > TryFrom < PrivateKeyInfo < Params , Key , PubKey > > for SecretDocument where Params : der :: Choice < 'a , Error = der :: Error > + Encode , Key : DecodeValue < 'a , Error = der :: Error > + FixedTag + 'a , Key : EncodeValue , PubKey : DecodeValue < 'a , Error = der :: Error > + FixedTag + 'a , PubKey : BitStringLike , { type Error = Error ; fn try_from (private_key : PrivateKeyInfo < Params , Key , PubKey >) -> Result < SecretDocument > { SecretDocument :: try_from (& private_key) } }
};
}
