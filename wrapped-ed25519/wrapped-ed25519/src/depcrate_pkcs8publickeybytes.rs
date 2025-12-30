// Generated macro for PublicKeyBytes (struct)
macro_rules! Depcrate_pkcs8PublicKeyBytes {
() => {
// Module: crate::pkcs8
// Provides: {"PublicKeyBytes"}
// Dependencies: {}
# [doc = " Ed25519 public key serialized as bytes."] # [doc = ""] # [doc = " This type is primarily useful for decoding/encoding SPKI public key"] # [doc = " files (either DER or PEM) encoded using the following traits:"] # [doc = ""] # [doc = " - [`DecodePublicKey`]: decode DER or PEM encoded PKCS#8 private key."] # [doc = " - [`EncodePublicKey`]: encode DER or PEM encoded PKCS#8 private key."] # [doc = ""] # [doc = " SPKI public key files encoded with PEM begin with:"] # [doc = ""] # [doc = " ```text"] # [doc = " -----BEGIN PUBLIC KEY-----"] # [doc = " ```"] # [doc = ""] # [doc = " Note that this type operates on raw bytes and performs no validation that"] # [doc = " public keys represent valid compressed Ed25519 y-coordinates."] # [derive (Clone , Copy , Eq , PartialEq)] pub struct PublicKeyBytes (pub [u8 ; Self :: BYTE_SIZE]) ;
};
}
