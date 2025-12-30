// Generated macro for DecodeRsaPublicKey (trait)
macro_rules! Depcrate_traitsDecodeRsaPublicKey {
() => {
// Module: crate::traits
// Provides: {"DecodeRsaPublicKey"}
// Dependencies: {}
# [doc = " Parse a [`RsaPublicKey`] from a PKCS#1-encoded document."] pub trait DecodeRsaPublicKey : Sized { # [doc = " Deserialize object from ASN.1 DER-encoded [`RsaPublicKey`]"] # [doc = " (binary format)."] fn from_pkcs1_der (bytes : & [u8]) -> Result < Self > ; # [doc = " Deserialize PEM-encoded [`RsaPublicKey`]."] # [doc = ""] # [doc = " Keys in this format begin with the following:"] # [doc = ""] # [doc = " ```text"] # [doc = " -----BEGIN RSA PUBLIC KEY-----"] # [doc = " ```"] # [cfg (feature = "pem")] fn from_pkcs1_pem (s : & str) -> Result < Self > { let (label , doc) = Document :: from_pem (s) ? ; RsaPublicKey :: validate_pem_label (label) ? ; Self :: from_pkcs1_der (doc . as_bytes ()) } # [doc = " Load [`RsaPublicKey`] from an ASN.1 DER-encoded file on the local"] # [doc = " filesystem (binary format)."] # [cfg (feature = "std")] fn read_pkcs1_der_file (path : impl AsRef < Path >) -> Result < Self > { let doc = Document :: read_der_file (path) ? ; Self :: from_pkcs1_der (doc . as_bytes ()) } # [doc = " Load [`RsaPublicKey`] from a PEM-encoded file on the local filesystem."] # [cfg (all (feature = "pem" , feature = "std"))] fn read_pkcs1_pem_file (path : impl AsRef < Path >) -> Result < Self > { let (label , doc) = Document :: read_pem_file (path) ? ; RsaPublicKey :: validate_pem_label (& label) ? ; Self :: from_pkcs1_der (doc . as_bytes ()) } }
};
}
