// Generated macro for PrivatePkcs1KeyDer (struct)
macro_rules! DepcratePrivatePkcs1KeyDer {
() => {
// Module: crate
// Provides: {"PrivatePkcs1KeyDer"}
// Dependencies: {}
# [doc = " A DER-encoded plaintext RSA private key; as specified in PKCS#1/RFC 3447"] # [doc = ""] # [doc = " RSA private keys are identified in PEM context as `RSA PRIVATE KEY` and when stored in a"] # [doc = " file usually use a `.pem` or `.key` extension."] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(all(feature = \"alloc\", feature = \"std\"))] {"] # [doc = " use rustls_pki_types::{PrivatePkcs1KeyDer, pem::PemObject};"] # [doc = ""] # [doc = " // load from a PEM file"] # [doc = " PrivatePkcs1KeyDer::from_pem_file(\"tests/data/rsa1024.pkcs1.pem\").unwrap();"] # [doc = ""] # [doc = " // or from a PEM byte slice..."] # [doc = " # let byte_slice = include_bytes!(\"../tests/data/rsa1024.pkcs1.pem\");"] # [doc = " PrivatePkcs1KeyDer::from_pem_slice(byte_slice).unwrap();"] # [doc = " # }"] # [doc = " ```"] # [derive (PartialEq , Eq)] pub struct PrivatePkcs1KeyDer < 'a > (Der < 'a >) ;
};
}
