// Generated macro for impl_1577 (impl)
macro_rules! Depcrate_x509impl_1577 {
() => {
// Module: crate::x509
// Provides: {"impl_1577"}
// Dependencies: {}
impl X509Req { # [doc = " A builder for `X509Req`."] pub fn builder () -> Result < X509ReqBuilder , ErrorStack > { X509ReqBuilder :: new () } from_pem ! { # [doc = " Deserializes a PEM-encoded PKCS#10 certificate request structure."] # [doc = ""] # [doc = " The input should have a header of `-----BEGIN CERTIFICATE REQUEST-----`."] # [doc = ""] # [doc = " This corresponds to [`PEM_read_bio_X509_REQ`]."] # [doc = ""] # [doc = " [`PEM_read_bio_X509_REQ`]: https://docs.openssl.org/master/man3/PEM_read_bio_X509_REQ/"] from_pem , X509Req , ffi :: PEM_read_bio_X509_REQ } from_der ! { # [doc = " Deserializes a DER-encoded PKCS#10 certificate request structure."] # [doc = ""] # [doc = " This corresponds to [`d2i_X509_REQ`]."] # [doc = ""] # [doc = " [`d2i_X509_REQ`]: https://docs.openssl.org/master/man3/d2i_X509_REQ/"] from_der , X509Req , ffi :: d2i_X509_REQ } }
};
}
