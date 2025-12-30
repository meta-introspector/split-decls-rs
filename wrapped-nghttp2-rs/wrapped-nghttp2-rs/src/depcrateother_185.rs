// Generated macro for other_185 (other)
macro_rules! Depcrateother_185 {
() => {
// Module: crate
// Provides: {"other_185"}
// Dependencies: {}
extern "C" { # [doc = " @function"] # [doc = ""] # [doc = " By default, nghttp2 library enforces subset of HTTP Messaging rules"] # [doc = " described in `HTTP/2 specification, section 8"] # [doc = " <https://tools.ietf.org/html/rfc7540#section-8>`_.  See"] # [doc = " :ref:`http-messaging` section for details.  For those applications"] # [doc = " who use nghttp2 library as non-HTTP use, give nonzero to |val| to"] # [doc = " disable this enforcement.  Please note that disabling this feature"] # [doc = " does not change the fundamental client and server model of HTTP."] # [doc = " That is, even if the validation is disabled, only client can send"] # [doc = " requests."] pub fn nghttp2_option_set_no_http_messaging (option : * mut nghttp2_option , val : :: std :: os :: raw :: c_int ,) ; }
};
}
