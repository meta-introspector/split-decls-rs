// Generated macro for impl_33 (impl)
macro_rules! Depcrate_private_key_infoimpl_33 {
() => {
// Module: crate::private_key_info
// Provides: {"impl_33"}
// Dependencies: {}
impl < 'a , Params , Key , PubKey > Sequence < 'a > for PrivateKeyInfo < Params , Key , PubKey > where Params : der :: Choice < 'a , Error = der :: Error > + Encode , Key : DecodeValue < 'a , Error = der :: Error > + FixedTag + 'a , Key : EncodeValue , PubKey : DecodeValue < 'a , Error = der :: Error > + FixedTag + 'a , PubKey : BitStringLike , { }
};
}
