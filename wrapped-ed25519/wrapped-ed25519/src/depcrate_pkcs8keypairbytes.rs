// Generated macro for KeypairBytes (struct)
macro_rules! Depcrate_pkcs8KeypairBytes {
() => {
// Module: crate::pkcs8
// Provides: {"KeypairBytes"}
// Dependencies: {}
# [doc = " Ed25519 keypair serialized as bytes."] # [doc = ""] # [doc = " This type is primarily useful for decoding/encoding PKCS#8 private key"] # [doc = " files (either DER or PEM) encoded using the following traits:"] # [doc = ""] # [doc = " - [`DecodePrivateKey`]: decode DER or PEM encoded PKCS#8 private key."] # [doc = " - [`EncodePrivateKey`]: encode DER or PEM encoded PKCS#8 private key."] # [doc = ""] # [doc = " PKCS#8 private key files encoded with PEM begin with:"] # [doc = ""] # [doc = " ```text"] # [doc = " -----BEGIN PRIVATE KEY-----"] # [doc = " ```"] # [doc = ""] # [doc = " Note that this type operates on raw bytes and performs no validation that"] # [doc = " keys represent valid Ed25519 field elements."] pub struct KeypairBytes { # [doc = " Ed25519 secret key."] # [doc = ""] # [doc = " Little endian serialization of an element of the Curve25519 scalar"] # [doc = " field, prior to \"clamping\" (i.e. setting/clearing bits to ensure the"] # [doc = " scalar is actually a valid field element)"] pub secret_key : [u8 ; Self :: BYTE_SIZE / 2] , # [doc = " Ed25519 public key (if available)."] # [doc = ""] # [doc = " Compressed Edwards-y encoded curve point."] pub public_key : Option < PublicKeyBytes > , }
};
}
