// Generated macro for macro_376 (macro)
macro_rules! Depcrate_ecdsamacro_376 {
() => {
// Module: crate::ecdsa
// Provides: {"macro_376"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , libressl , boringssl , awslc))] { use ffi :: { ECDSA_SIG_set0 , ECDSA_SIG_get0 } ; } else { # [allow (bad_style)] unsafe fn ECDSA_SIG_set0 (sig : * mut ffi :: ECDSA_SIG , r : * mut ffi :: BIGNUM , s : * mut ffi :: BIGNUM ,) -> c_int { if r . is_null () || s . is_null () { return 0 ; } ffi :: BN_clear_free ((* sig) . r) ; ffi :: BN_clear_free ((* sig) . s) ; (* sig) . r = r ; (* sig) . s = s ; 1 } # [allow (bad_style)] unsafe fn ECDSA_SIG_get0 (sig : * const ffi :: ECDSA_SIG , pr : * mut * const ffi :: BIGNUM , ps : * mut * const ffi :: BIGNUM) { if ! pr . is_null () { (* pr) = (* sig) . r ; } if ! ps . is_null () { (* ps) = (* sig) . s ; } } } }
};
}
