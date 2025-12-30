// Generated macro for impl_30 (impl)
macro_rules! Depcrate_private_key_infoimpl_30 {
() => {
// Module: crate::private_key_info
// Provides: {"impl_30"}
// Dependencies: {}
impl < 'a , Params , Key , PubKey > PrivateKeyInfo < Params , Key , PubKey > where Params : der :: Choice < 'a > + Encode , PubKey : BitStringLike , { # [doc = " Get a `BIT STRING` representation of the public key, if present."] fn public_key_bit_string (& self) -> Option < ContextSpecific < BitStringRef < '_ > > > { self . public_key . as_ref () . map (| pk | { let value = pk . as_bit_string () ; ContextSpecific { tag_number : PUBLIC_KEY_TAG , tag_mode : TagMode :: Implicit , value , } }) } }
};
}
