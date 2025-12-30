// Generated macro for impl_115 (impl)
macro_rules! Depcrate_agreementimpl_115 {
() => {
// Module: crate::agreement
// Provides: {"impl_115"}
// Dependencies: {}
impl AsBigEndian < EcPublicKeyUncompressedBin < 'static > > for PublicKey { # [doc = " Provides the public key elliptic curve point to a compressed point format."] # [doc = ""] # [doc = " Equivalent to [`PublicKey::as_ref`] for ECDH key types, except that it provides you a copy instead of a reference."] # [doc = ""] # [doc = " # Errors"] # [doc = " Returns an error if the underlying implementation is unable to marshal the public key to this format."] fn as_be_bytes (& self ,) -> Result < EcPublicKeyUncompressedBin < 'static > , crate :: error :: Unspecified > { if self . algorithm () . id == AlgorithmID :: X25519 { return Err (Unspecified) ; } let mut buffer = vec ! [0u8 ; self . len] ; buffer . copy_from_slice (& self . key_bytes [0 .. self . len]) ; Ok (EcPublicKeyUncompressedBin :: new (buffer)) } }
};
}
