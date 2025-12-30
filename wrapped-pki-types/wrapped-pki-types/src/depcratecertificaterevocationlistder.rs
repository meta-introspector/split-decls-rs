// Generated macro for CertificateRevocationListDer (struct)
macro_rules! DepcrateCertificateRevocationListDer {
() => {
// Module: crate
// Provides: {"CertificateRevocationListDer"}
// Dependencies: {}
# [doc = " A Certificate Revocation List; as specified in RFC 5280"] # [doc = ""] # [doc = " Certificate revocation lists are identified in PEM context as `X509 CRL` and when stored in a"] # [doc = " file usually use a `.crl` extension. For more on PEM files, refer to the crate documentation."] # [doc = ""] # [doc = " ```rust"] # [doc = " # #[cfg(all(feature = \"alloc\", feature = \"std\"))] {"] # [doc = " use rustls_pki_types::{CertificateRevocationListDer, pem::PemObject};"] # [doc = ""] # [doc = " // load several from a PEM file"] # [doc = " let crls: Vec<_> = CertificateRevocationListDer::pem_file_iter(\"tests/data/crl.pem\")"] # [doc = "     .unwrap()"] # [doc = "     .collect();"] # [doc = " assert!(crls.len() >= 1);"] # [doc = ""] # [doc = " // or one from a PEM byte slice..."] # [doc = " # let byte_slice = include_bytes!(\"../tests/data/crl.pem\");"] # [doc = " CertificateRevocationListDer::from_pem_slice(byte_slice).unwrap();"] # [doc = ""] # [doc = " // or several from a PEM byte slice"] # [doc = " let crls: Vec<_> = CertificateRevocationListDer::pem_slice_iter(byte_slice)"] # [doc = "     .collect();"] # [doc = " assert!(crls.len() >= 1);"] # [doc = " # }"] # [doc = " ```"] # [derive (Clone , Debug , Hash , PartialEq , Eq)] pub struct CertificateRevocationListDer < 'a > (Der < 'a >) ;
};
}
