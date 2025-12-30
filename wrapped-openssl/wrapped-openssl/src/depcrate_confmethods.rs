// Generated macro for methods (module)
macro_rules! Depcrate_confmethods {
() => {
// Module: crate::conf
// Provides: {"methods"}
// Dependencies: {}
# [cfg (not (any (boringssl , libressl400 , awslc)))] mod methods { use super :: Conf ; use crate :: cvt_p ; use crate :: error :: ErrorStack ; use openssl_macros :: corresponds ; pub struct ConfMethod (* mut ffi :: CONF_METHOD) ; impl ConfMethod { # [doc = " Retrieve handle to the default OpenSSL configuration file processing function."] # [corresponds (NCONF_default)] # [allow (clippy :: should_implement_trait)] pub fn default () -> ConfMethod { unsafe { ffi :: init () ; ConfMethod (ffi :: NCONF_default ()) } } # [doc = " Construct from raw pointer."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " The caller must ensure that the pointer is valid."] pub unsafe fn from_ptr (ptr : * mut ffi :: CONF_METHOD) -> ConfMethod { ConfMethod (ptr) } # [doc = " Convert to raw pointer."] pub fn as_ptr (& self) -> * mut ffi :: CONF_METHOD { self . 0 } } impl Conf { # [doc = " Create a configuration parser."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use openssl::conf::{Conf, ConfMethod};"] # [doc = ""] # [doc = " let conf = Conf::new(ConfMethod::default());"] # [doc = " ```"] # [corresponds (NCONF_new)] pub fn new (method : ConfMethod) -> Result < Conf , ErrorStack > { unsafe { cvt_p (ffi :: NCONF_new (method . as_ptr ())) . map (Conf) } } } }
};
}
