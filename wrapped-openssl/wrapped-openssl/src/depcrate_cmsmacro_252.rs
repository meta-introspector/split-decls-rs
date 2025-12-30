// Generated macro for macro_252 (macro)
macro_rules! Depcrate_cmsmacro_252 {
() => {
// Module: crate::cms
// Provides: {"macro_252"}
// Dependencies: {}
bitflags ! { # [derive (Copy , Clone , Debug , Eq , Hash , Ord , PartialEq , PartialOrd)] # [repr (transparent)] pub struct CMSOptions : c_uint { const TEXT = ffi :: CMS_TEXT ; const CMS_NOCERTS = ffi :: CMS_NOCERTS ; const NO_CONTENT_VERIFY = ffi :: CMS_NO_CONTENT_VERIFY ; const NO_ATTR_VERIFY = ffi :: CMS_NO_ATTR_VERIFY ; const NOSIGS = ffi :: CMS_NOSIGS ; const NOINTERN = ffi :: CMS_NOINTERN ; const NO_SIGNER_CERT_VERIFY = ffi :: CMS_NO_SIGNER_CERT_VERIFY ; const NOVERIFY = ffi :: CMS_NOVERIFY ; const DETACHED = ffi :: CMS_DETACHED ; const BINARY = ffi :: CMS_BINARY ; const NOATTR = ffi :: CMS_NOATTR ; const NOSMIMECAP = ffi :: CMS_NOSMIMECAP ; const NOOLDMIMETYPE = ffi :: CMS_NOOLDMIMETYPE ; const CRLFEOL = ffi :: CMS_CRLFEOL ; const STREAM = ffi :: CMS_STREAM ; const NOCRL = ffi :: CMS_NOCRL ; const PARTIAL = ffi :: CMS_PARTIAL ; const REUSE_DIGEST = ffi :: CMS_REUSE_DIGEST ; const USE_KEYID = ffi :: CMS_USE_KEYID ; const DEBUG_DECRYPT = ffi :: CMS_DEBUG_DECRYPT ; # [cfg (any (ossl102 , libressl))] const KEY_PARAM = ffi :: CMS_KEY_PARAM ; # [cfg (any (ossl110 , libressl))] const ASCIICRLF = ffi :: CMS_ASCIICRLF ; } }
};
}
