// Generated macro for EncodeRsaPrivateKey (trait)
macro_rules! Depcrate_traitsEncodeRsaPrivateKey {
() => {
// Module: crate::traits
// Provides: {"EncodeRsaPrivateKey"}
// Dependencies: {}
# [doc = " Serialize a [`RsaPrivateKey`] to a PKCS#1 encoded document."] # [cfg (feature = "alloc")] pub trait EncodeRsaPrivateKey { # [doc = " Serialize a [`SecretDocument`] containing a PKCS#1-encoded private key."] fn to_pkcs1_der (& self) -> Result < SecretDocument > ; # [doc = " Serialize this private key as PEM-encoded PKCS#1 with the given [`LineEnding`]."] # [cfg (feature = "pem")] fn to_pkcs1_pem (& self , line_ending : LineEnding) -> Result < Zeroizing < String > > { let doc = self . to_pkcs1_der () ? ; Ok (doc . to_pem (RsaPrivateKey :: PEM_LABEL , line_ending) ?) } # [doc = " Write ASN.1 DER-encoded PKCS#1 private key to the given path."] # [cfg (feature = "std")] fn write_pkcs1_der_file (& self , path : impl AsRef < Path >) -> Result < () > { Ok (self . to_pkcs1_der () ? . write_der_file (path) ?) } # [doc = " Write ASN.1 PEM-encoded PKCS#1 private key to the given path."] # [cfg (all (feature = "pem" , feature = "std"))] fn write_pkcs1_pem_file (& self , path : impl AsRef < Path > , line_ending : LineEnding) -> Result < () > { let doc = self . to_pkcs1_der () ? ; Ok (doc . write_pem_file (path , RsaPrivateKey :: PEM_LABEL , line_ending) ?) } }
};
}
