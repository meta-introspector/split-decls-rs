// Generated macro for impl_1594 (impl)
macro_rules! Depcrate_x509impl_1594 {
() => {
// Module: crate::x509
// Provides: {"impl_1594"}
// Dependencies: {}
impl X509Crl { from_pem ! { # [doc = " Deserializes a PEM-encoded Certificate Revocation List"] # [doc = ""] # [doc = " The input should have a header of `-----BEGIN X509 CRL-----`."] # [corresponds (PEM_read_bio_X509_CRL)] from_pem , X509Crl , ffi :: PEM_read_bio_X509_CRL } from_der ! { # [doc = " Deserializes a DER-encoded Certificate Revocation List"] # [corresponds (d2i_X509_CRL)] from_der , X509Crl , ffi :: d2i_X509_CRL } }
};
}
