// Generated macro for macro_291 (macro)
macro_rules! Depcrate_dhmacro_291 {
() => {
// Module: crate::dh
// Provides: {"macro_291"}
// Dependencies: {}
cfg_if ! { if # [cfg (any (ossl110 , libressl , boringssl , awslc))] { use ffi :: { DH_set0_pqg , DH_get0_pqg , DH_get0_key , DH_set0_key } ; } else { # [allow (bad_style)] unsafe fn DH_set0_pqg (dh : * mut ffi :: DH , p : * mut ffi :: BIGNUM , q : * mut ffi :: BIGNUM , g : * mut ffi :: BIGNUM ,) -> :: libc :: c_int { (* dh) . p = p ; (* dh) . q = q ; (* dh) . g = g ; 1 } # [allow (bad_style)] unsafe fn DH_get0_pqg (dh : * mut ffi :: DH , p : * mut * const ffi :: BIGNUM , q : * mut * const ffi :: BIGNUM , g : * mut * const ffi :: BIGNUM ,) { if ! p . is_null () { * p = (* dh) . p ; } if ! q . is_null () { * q = (* dh) . q ; } if ! g . is_null () { * g = (* dh) . g ; } } # [allow (bad_style)] unsafe fn DH_set0_key (dh : * mut ffi :: DH , pub_key : * mut ffi :: BIGNUM , priv_key : * mut ffi :: BIGNUM ,) -> :: libc :: c_int { (* dh) . pub_key = pub_key ; (* dh) . priv_key = priv_key ; 1 } # [allow (bad_style)] unsafe fn DH_get0_key (dh : * mut ffi :: DH , pub_key : * mut * const ffi :: BIGNUM , priv_key : * mut * const ffi :: BIGNUM ,) { if ! pub_key . is_null () { * pub_key = (* dh) . pub_key ; } if ! priv_key . is_null () { * priv_key = (* dh) . priv_key ; } } } }
};
}
