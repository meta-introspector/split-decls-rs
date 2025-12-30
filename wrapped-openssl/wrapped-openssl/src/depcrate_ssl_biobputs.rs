// Generated macro for bputs (function)
macro_rules! Depcrate_ssl_biobputs {
() => {
// Module: crate::ssl::bio
// Provides: {"bputs"}
// Dependencies: {}
unsafe extern "C" fn bputs < S : Write > (bio : * mut BIO , s : * const c_char) -> c_int { bwrite :: < S > (bio , s , strlen (s) as c_int) }
};
}
