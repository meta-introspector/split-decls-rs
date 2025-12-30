// Generated macro for macro_318 (macro)
macro_rules! Depcrate_dsamacro_318 {
() => {
// Module: crate::dsa
// Provides: {"macro_318"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , libressl , boringssl , awslc))] { use ffi :: { DSA_get0_key , DSA_get0_pqg , DSA_set0_key , DSA_set0_pqg } ; } else { # [allow (bad_style)] unsafe fn DSA_get0_pqg (d : * mut ffi :: DSA , p : * mut * const ffi :: BIGNUM , q : * mut * const ffi :: BIGNUM , g : * mut * const ffi :: BIGNUM) { if ! p . is_null () { * p = (* d) . p ; } if ! q . is_null () { * q = (* d) . q ; } if ! g . is_null () { * g = (* d) . g ; } } # [allow (bad_style)] unsafe fn DSA_get0_key (d : * mut ffi :: DSA , pub_key : * mut * const ffi :: BIGNUM , priv_key : * mut * const ffi :: BIGNUM) { if ! pub_key . is_null () { * pub_key = (* d) . pub_key ; } if ! priv_key . is_null () { * priv_key = (* d) . priv_key ; } } # [allow (bad_style)] unsafe fn DSA_set0_key (d : * mut ffi :: DSA , pub_key : * mut ffi :: BIGNUM , priv_key : * mut ffi :: BIGNUM) -> c_int { (* d) . pub_key = pub_key ; (* d) . priv_key = priv_key ; 1 } # [allow (bad_style)] unsafe fn DSA_set0_pqg (d : * mut ffi :: DSA , p : * mut ffi :: BIGNUM , q : * mut ffi :: BIGNUM , g : * mut ffi :: BIGNUM) -> c_int { (* d) . p = p ; (* d) . q = q ; (* d) . g = g ; 1 } } }
};
}
