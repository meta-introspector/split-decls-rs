// Generated macro for EncodeRsaPublicKey (trait)
macro_rules! Depcrate_traitsEncodeRsaPublicKey {
() => {
// Module: crate::traits
// Provides: {"EncodeRsaPublicKey"}
// Dependencies: {}
# [doc = " Serialize a [`RsaPublicKey`] to a PKCS#1-encoded document."] # [cfg (feature = "alloc")] pub trait EncodeRsaPublicKey { # [doc = " Serialize a [`Document`] containing a PKCS#1-encoded public key."] fn to_pkcs1_der (& self) -> Result < Document > ; # [doc = " Serialize this public key as PEM-encoded PKCS#1 with the given line ending."] # [cfg (feature = "pem")] fn to_pkcs1_pem (& self , line_ending : LineEnding) -> Result < String > { let doc = self . to_pkcs1_der () ? ; Ok (doc . to_pem (RsaPublicKey :: PEM_LABEL , line_ending) ?) } # [doc = " Write ASN.1 DER-encoded public key to the given path."] # [cfg (feature = "std")] fn write_pkcs1_der_file (& self , path : impl AsRef < Path >) -> Result < () > { Ok (self . to_pkcs1_der () ? . write_der_file (path) ?) } # [doc = " Write ASN.1 PEM-encoded public key to the given path."] # [cfg (all (feature = "pem" , feature = "std"))] fn write_pkcs1_pem_file (& self , path : impl AsRef < Path > , line_ending : LineEnding) -> Result < () > { let doc = self . to_pkcs1_der () ? ; Ok (doc . write_pem_file (path , RsaPublicKey :: PEM_LABEL , line_ending) ?) } }
};
}
