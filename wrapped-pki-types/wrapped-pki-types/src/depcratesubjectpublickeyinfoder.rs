// Generated macro for SubjectPublicKeyInfoDer (struct)
macro_rules! DepcrateSubjectPublicKeyInfoDer {
() => {
// Module: crate
// Provides: {"SubjectPublicKeyInfoDer"}
// Dependencies: {}
# [doc = " A DER-encoded SubjectPublicKeyInfo (SPKI), as specified in RFC 5280."] # [doc = ""] # [doc = " Public keys are identified in PEM context as a `PUBLIC KEY`."] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(all(feature = \"alloc\", feature = \"std\"))] {"] # [doc = " use rustls_pki_types::{SubjectPublicKeyInfoDer, pem::PemObject};"] # [doc = ""] # [doc = " // load from a PEM file"] # [doc = " SubjectPublicKeyInfoDer::from_pem_file(\"tests/data/spki.pem\").unwrap();"] # [doc = ""] # [doc = " // or from a PEM byte slice..."] # [doc = " # let byte_slice = include_bytes!(\"../tests/data/spki.pem\");"] # [doc = " SubjectPublicKeyInfoDer::from_pem_slice(byte_slice).unwrap();"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Debug , Hash , PartialEq , Eq)] pub struct SubjectPublicKeyInfoDer < 'a > (Der < 'a >) ;
};
}
