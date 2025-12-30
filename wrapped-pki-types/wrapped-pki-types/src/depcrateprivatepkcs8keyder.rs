// Generated macro for PrivatePkcs8KeyDer (struct)
macro_rules! DepcratePrivatePkcs8KeyDer {
() => {
// Module: crate
// Provides: {"PrivatePkcs8KeyDer"}
// Dependencies: {}
# [doc = " A DER-encoded plaintext private key; as specified in PKCS#8/RFC 5958"] # [doc = ""] # [doc = " PKCS#8 private keys are identified in PEM context as `PRIVATE KEY` and when stored in a"] # [doc = " file usually use a `.pem` or `.key` extension. For more on PEM files, refer to the crate"] # [doc = " documentation."] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(all(feature = \"alloc\", feature = \"std\"))] {"] # [doc = " use rustls_pki_types::{PrivatePkcs8KeyDer, pem::PemObject};"] # [doc = ""] # [doc = " // load from a PEM file"] # [doc = " PrivatePkcs8KeyDer::from_pem_file(\"tests/data/nistp256key.pkcs8.pem\").unwrap();"] # [doc = " PrivatePkcs8KeyDer::from_pem_file(\"tests/data/rsa1024.pkcs8.pem\").unwrap();"] # [doc = ""] # [doc = " // or from a PEM byte slice..."] # [doc = " # let byte_slice = include_bytes!(\"../tests/data/nistp256key.pkcs8.pem\");"] # [doc = " PrivatePkcs8KeyDer::from_pem_slice(byte_slice).unwrap();"] # [doc = " # }"] # [doc = " ```"] # [derive (PartialEq , Eq)] pub struct PrivatePkcs8KeyDer < 'a > (Der < 'a >) ;
};
}
