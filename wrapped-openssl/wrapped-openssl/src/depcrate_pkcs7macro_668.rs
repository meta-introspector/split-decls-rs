// Generated macro for macro_668 (macro)
macro_rules! Depcrate_pkcs7macro_668 {
() => {
// Module: crate::pkcs7
// Provides: {"macro_668"}
// Dependencies: {}
bitflags ! { # [derive (Copy , Clone , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct Pkcs7Flags : c_int { const TEXT = ffi :: PKCS7_TEXT ; const NOCERTS = ffi :: PKCS7_NOCERTS ; const NOSIGS = ffi :: PKCS7_NOSIGS ; const NOCHAIN = ffi :: PKCS7_NOCHAIN ; const NOINTERN = ffi :: PKCS7_NOINTERN ; const NOVERIFY = ffi :: PKCS7_NOVERIFY ; const DETACHED = ffi :: PKCS7_DETACHED ; const BINARY = ffi :: PKCS7_BINARY ; const NOATTR = ffi :: PKCS7_NOATTR ; const NOSMIMECAP = ffi :: PKCS7_NOSMIMECAP ; const NOOLDMIMETYPE = ffi :: PKCS7_NOOLDMIMETYPE ; const CRLFEOL = ffi :: PKCS7_CRLFEOL ; const STREAM = ffi :: PKCS7_STREAM ; const NOCRL = ffi :: PKCS7_NOCRL ; const PARTIAL = ffi :: PKCS7_PARTIAL ; const REUSE_DIGEST = ffi :: PKCS7_REUSE_DIGEST ; # [cfg (ossl110)] const NO_DUAL_CONTENT = ffi :: PKCS7_NO_DUAL_CONTENT ; } }
};
}
