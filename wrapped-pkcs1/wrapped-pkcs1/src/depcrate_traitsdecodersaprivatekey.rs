// Generated macro for DecodeRsaPrivateKey (trait)
macro_rules! Depcrate_traitsDecodeRsaPrivateKey {
() => {
// Module: crate::traits
// Provides: {"DecodeRsaPrivateKey"}
// Dependencies: {}
# [doc = " Parse an [`RsaPrivateKey`] from a PKCS#1-encoded document."] pub trait DecodeRsaPrivateKey : Sized { # [doc = " Deserialize PKCS#1 private key from ASN.1 DER-encoded data"] # [doc = " (binary format)."] fn from_pkcs1_der (bytes : & [u8]) -> Result < Self > ; # [doc = " Deserialize PKCS#1-encoded private key from PEM."] # [doc = ""] # [doc = " Keys in this format begin with the following:"] # [doc = ""] # [doc = " ```text"] # [doc = " -----BEGIN RSA PRIVATE KEY-----"] # [doc = " ```"] # [cfg (feature = "pem")] fn from_pkcs1_pem (s : & str) -> Result < Self > { let (label , doc) = SecretDocument :: from_pem (s) ? ; RsaPrivateKey :: validate_pem_label (label) ? ; Self :: from_pkcs1_der (doc . as_bytes ()) } # [doc = " Load PKCS#1 private key from an ASN.1 DER-encoded file on the local"] # [doc = " filesystem (binary format)."] # [cfg (feature = "std")] fn read_pkcs1_der_file (path : impl AsRef < Path >) -> Result < Self > { Self :: from_pkcs1_der (SecretDocument :: read_der_file (path) ? . as_bytes ()) } # [doc = " Load PKCS#1 private key from a PEM-encoded file on the local filesystem."] # [cfg (all (feature = "pem" , feature = "std"))] fn read_pkcs1_pem_file (path : impl AsRef < Path >) -> Result < Self > { let (label , doc) = SecretDocument :: read_pem_file (path) ? ; RsaPrivateKey :: validate_pem_label (& label) ? ; Self :: from_pkcs1_der (doc . as_bytes ()) } }
};
}
