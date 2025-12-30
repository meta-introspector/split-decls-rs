// Generated macro for CertificateSigningRequestDer (struct)
macro_rules! DepcrateCertificateSigningRequestDer {
() => {
// Module: crate
// Provides: {"CertificateSigningRequestDer"}
// Dependencies: {}
# [doc = " A Certificate Signing Request; as specified in RFC 2986"] # [doc = ""] # [doc = " Certificate signing requests are identified in PEM context as `CERTIFICATE REQUEST` and when stored in a"] # [doc = " file usually use a `.csr` extension. For more on PEM files, refer to the crate documentation."] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(all(feature = \"alloc\", feature = \"std\"))] {"] # [doc = " use rustls_pki_types::{CertificateSigningRequestDer, pem::PemObject};"] # [doc = ""] # [doc = " // load from a PEM file"] # [doc = " CertificateSigningRequestDer::from_pem_file(\"tests/data/csr.pem\").unwrap();"] # [doc = ""] # [doc = " // or from a PEM byte slice..."] # [doc = " # let byte_slice = include_bytes!(\"../tests/data/csr.pem\");"] # [doc = " CertificateSigningRequestDer::from_pem_slice(byte_slice).unwrap();"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Debug , Hash , PartialEq , Eq)] pub struct CertificateSigningRequestDer < 'a > (Der < 'a >) ;
};
}
