// Generated macro for destroy (function)
macro_rules! Depcrate_ssl_biodestroy {
() => {
// Module: crate::ssl::bio
// Provides: {"destroy"}
// Dependencies: {}
unsafe extern "C" fn destroy < S > (bio : * mut BIO) -> c_int { if bio . is_null () { return 0 ; } let data = BIO_get_data (bio) ; assert ! (! data . is_null ()) ; let _ = Box :: < StreamState < S > > :: from_raw (data as * mut _) ; BIO_set_data (bio , ptr :: null_mut ()) ; BIO_set_init (bio , 0) ; 1 }
};
}
