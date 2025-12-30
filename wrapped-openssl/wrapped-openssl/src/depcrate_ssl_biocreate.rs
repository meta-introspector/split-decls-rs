// Generated macro for create (function)
macro_rules! Depcrate_ssl_biocreate {
() => {
// Module: crate::ssl::bio
// Provides: {"create"}
// Dependencies: {}
unsafe extern "C" fn create (bio : * mut BIO) -> c_int { BIO_set_init (bio , 0) ; BIO_set_num (bio , 0) ; BIO_set_data (bio , ptr :: null_mut ()) ; BIO_set_flags (bio , 0) ; 1 }
};
}
