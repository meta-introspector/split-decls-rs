// Generated macro for new (function)
macro_rules! Depcrate_ssl_bionew {
() => {
// Module: crate::ssl::bio
// Provides: {"new"}
// Dependencies: {}
pub fn new < S : Read + Write > (stream : S) -> Result < (* mut BIO , BioMethod) , ErrorStack > { let method = BioMethod :: new :: < S > () ? ; let state = Box :: new (StreamState { stream , error : None , panic : None , dtls_mtu_size : 0 , }) ; unsafe { let bio = cvt_p (BIO_new (method . 0 . get ())) ? ; BIO_set_data (bio , Box :: into_raw (state) as * mut _) ; BIO_set_init (bio , 1) ; Ok ((bio , method)) } }
};
}
