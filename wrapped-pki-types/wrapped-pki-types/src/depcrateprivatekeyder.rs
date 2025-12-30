// Generated macro for PrivateKeyDer (enum)
macro_rules! DepcratePrivateKeyDer {
() => {
// Module: crate
// Provides: {"PrivateKeyDer"}
// Dependencies: {}
# [doc = " A DER-encoded X.509 private key, in one of several formats"] # [doc = ""] # [doc = " See variant inner types for more detailed information."] # [doc = ""] # [doc = " This can load several types of PEM-encoded private key, and then reveal"] # [doc = " which types were found:"] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(all(feature = \"alloc\", feature = \"std\"))] {"] # [doc = " use rustls_pki_types::{PrivateKeyDer, pem::PemObject};"] # [doc = ""] # [doc = " // load from a PEM file"] # [doc = " let pkcs8 = PrivateKeyDer::from_pem_file(\"tests/data/nistp256key.pkcs8.pem\").unwrap();"] # [doc = " let pkcs1 = PrivateKeyDer::from_pem_file(\"tests/data/rsa1024.pkcs1.pem\").unwrap();"] # [doc = " let sec1 = PrivateKeyDer::from_pem_file(\"tests/data/nistp256key.pem\").unwrap();"] # [doc = " assert!(matches!(pkcs8, PrivateKeyDer::Pkcs8(_)));"] # [doc = " assert!(matches!(pkcs1, PrivateKeyDer::Pkcs1(_)));"] # [doc = " assert!(matches!(sec1, PrivateKeyDer::Sec1(_)));"] # [doc = " # }"] # [doc = " ```"] # [non_exhaustive] # [derive (Debug , PartialEq , Eq)] pub enum PrivateKeyDer < 'a > { # [doc = " An RSA private key"] Pkcs1 (PrivatePkcs1KeyDer < 'a >) , # [doc = " A Sec1 private key"] Sec1 (PrivateSec1KeyDer < 'a >) , # [doc = " A PKCS#8 private key"] Pkcs8 (PrivatePkcs8KeyDer < 'a >) , }
};
}
