// Generated macro for impl_114 (impl)
macro_rules! Depcrate_agreementimpl_114 {
() => {
// Module: crate::agreement
// Provides: {"impl_114"}
// Dependencies: {}
impl AsBigEndian < EcPublicKeyCompressedBin < 'static > > for PublicKey { # [doc = " Provides the public key elliptic curve point to a compressed point format."] # [doc = " # Errors"] # [doc = " Returns an error if the underlying implementation is unable to marshal the public key to this format."] fn as_be_bytes (& self) -> Result < EcPublicKeyCompressedBin < 'static > , crate :: error :: Unspecified > { let evp_pkey = match & self . inner_key { KeyInner :: ECDH_P256 (evp_pkey) | KeyInner :: ECDH_P384 (evp_pkey) | KeyInner :: ECDH_P521 (evp_pkey) => evp_pkey , KeyInner :: X25519 (_) => return Err (Unspecified) , } ; let pub_point = marshal_sec1_public_point (evp_pkey , true) ? ; Ok (EcPublicKeyCompressedBin :: new (pub_point)) } }
};
}
