// Generated macro for PrivateSec1KeyDer (struct)
macro_rules! DepcratePrivateSec1KeyDer {
() => {
// Module: crate
// Provides: {"PrivateSec1KeyDer"}
// Dependencies: {}
# [doc = " A Sec1-encoded plaintext private key; as specified in RFC 5915"] # [doc = ""] # [doc = " Sec1 private keys are identified in PEM context as `EC PRIVATE KEY` and when stored in a"] # [doc = " file usually use a `.pem` or `.key` extension. For more on PEM files, refer to the crate"] # [doc = " documentation."] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(all(feature = \"alloc\", feature = \"std\"))] {"] # [doc = " use rustls_pki_types::{PrivateSec1KeyDer, pem::PemObject};"] # [doc = ""] # [doc = " // load from a PEM file"] # [doc = " PrivateSec1KeyDer::from_pem_file(\"tests/data/nistp256key.pem\").unwrap();"] # [doc = ""] # [doc = " // or from a PEM byte slice..."] # [doc = " # let byte_slice = include_bytes!(\"../tests/data/nistp256key.pem\");"] # [doc = " PrivateSec1KeyDer::from_pem_slice(byte_slice).unwrap();"] # [doc = " # }"] # [doc = " ```"] # [derive (PartialEq , Eq)] pub struct PrivateSec1KeyDer < 'a > (Der < 'a >) ;
};
}
