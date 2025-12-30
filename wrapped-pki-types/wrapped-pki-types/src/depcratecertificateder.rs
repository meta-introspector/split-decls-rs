// Generated macro for CertificateDer (struct)
macro_rules! DepcrateCertificateDer {
() => {
// Module: crate
// Provides: {"CertificateDer"}
// Dependencies: {}
# [doc = " A DER-encoded X.509 certificate; as specified in RFC 5280"] # [doc = ""] # [doc = " Certificates are identified in PEM context as `CERTIFICATE` and when stored in a"] # [doc = " file usually use a `.pem`, `.cer` or `.crt` extension. For more on PEM files, refer to the"] # [doc = " crate documentation."] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(all(feature = \"alloc\", feature = \"std\"))] {"] # [doc = " use rustls_pki_types::{CertificateDer, pem::PemObject};"] # [doc = ""] # [doc = " // load several from a PEM file"] # [doc = " let certs: Vec<_> = CertificateDer::pem_file_iter(\"tests/data/certificate.chain.pem\")"] # [doc = "     .unwrap()"] # [doc = "     .collect();"] # [doc = " assert_eq!(certs.len(), 3);"] # [doc = ""] # [doc = " // or one from a PEM byte slice..."] # [doc = " # let byte_slice = include_bytes!(\"../tests/data/certificate.chain.pem\");"] # [doc = " CertificateDer::from_pem_slice(byte_slice).unwrap();"] # [doc = ""] # [doc = " // or several from a PEM byte slice"] # [doc = " let certs: Vec<_> = CertificateDer::pem_slice_iter(byte_slice)"] # [doc = "     .collect();"] # [doc = " assert_eq!(certs.len(), 3);"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Debug , Hash , PartialEq , Eq)] pub struct CertificateDer < 'a > (Der < 'a >) ;
};
}
