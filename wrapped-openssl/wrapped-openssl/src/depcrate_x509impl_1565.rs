// Generated macro for impl_1565 (impl)
macro_rules! Depcrate_x509impl_1565 {
() => {
// Module: crate::x509
// Provides: {"impl_1565"}
// Dependencies: {}
impl X509Name { # [doc = " Returns a new builder."] pub fn builder () -> Result < X509NameBuilder , ErrorStack > { X509NameBuilder :: new () } # [doc = " Loads subject names from a file containing PEM-formatted certificates."] # [doc = ""] # [doc = " This is commonly used in conjunction with `SslContextBuilder::set_client_ca_list`."] pub fn load_client_ca_file < P : AsRef < Path > > (file : P) -> Result < Stack < X509Name > , ErrorStack > { let file = CString :: new (file . as_ref () . as_os_str () . to_str () . unwrap ()) . unwrap () ; unsafe { cvt_p (ffi :: SSL_load_client_CA_file (file . as_ptr ())) . map (| p | Stack :: from_ptr (p)) } } from_der ! { # [doc = " Deserializes a DER-encoded X509 name structure."] # [doc = ""] # [doc = " This corresponds to [`d2i_X509_NAME`]."] # [doc = ""] # [doc = " [`d2i_X509_NAME`]: https://docs.openssl.org/master/man3/d2i_X509_NAME/"] from_der , X509Name , ffi :: d2i_X509_NAME } }
};
}
