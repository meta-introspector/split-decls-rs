// Generated macro for impl_31 (impl)
macro_rules! Depcrate_private_key_infoimpl_31 {
() => {
// Module: crate::private_key_info
// Provides: {"impl_31"}
// Dependencies: {}
impl < 'a , Params , Key , PubKey > DecodeValue < 'a > for PrivateKeyInfo < Params , Key , PubKey > where Params : der :: Choice < 'a , Error = der :: Error > + Encode , Key : DecodeValue < 'a , Error = der :: Error > + FixedTag + 'a , PubKey : DecodeValue < 'a , Error = der :: Error > + FixedTag + 'a , { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : Header) -> der :: Result < Self > { let version = Version :: decode (reader) ? ; let algorithm = reader . decode () ? ; let private_key = Key :: decode (reader) ? ; let _attributes = reader . context_specific :: < SequenceRef < '_ > > (ATTRIBUTES_TAG , TagMode :: Implicit) ? ; let public_key = reader . context_specific :: < PubKey > (PUBLIC_KEY_TAG , TagMode :: Implicit) ? ; if version . has_public_key () != public_key . is_some () { return Err (reader . error (der :: Tag :: ContextSpecific { constructed : true , number : PUBLIC_KEY_TAG , } . value_error () ,)) ; } while ! reader . is_finished () { reader . decode :: < ContextSpecific < AnyRef < '_ > > > () ? ; } Ok (Self { algorithm , private_key , public_key , }) } }
};
}
