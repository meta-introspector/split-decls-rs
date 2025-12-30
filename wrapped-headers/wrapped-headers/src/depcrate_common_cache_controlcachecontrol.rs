// Generated macro for CacheControl (struct)
macro_rules! Depcrate_common_cache_controlCacheControl {
() => {
// Module: crate::common::cache_control
// Provides: {"CacheControl"}
// Dependencies: {}
# [doc = " `Cache-Control` header, defined in [RFC7234](https://tools.ietf.org/html/rfc7234#section-5.2)"] # [doc = " with extensions in [RFC8246](https://www.rfc-editor.org/rfc/rfc8246)"] # [doc = ""] # [doc = " The `Cache-Control` header field is used to specify directives for"] # [doc = " caches along the request/response chain.  Such cache directives are"] # [doc = " unidirectional in that the presence of a directive in a request does"] # [doc = " not imply that the same directive is to be given in the response."] # [doc = ""] # [doc = " ## ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Cache-Control   = 1#cache-directive"] # [doc = " cache-directive = token [ \"=\" ( token / quoted-string ) ]"] # [doc = " ```"] # [doc = ""] # [doc = " ## Example values"] # [doc = ""] # [doc = " * `no-cache`"] # [doc = " * `private, community=\"UCI\"`"] # [doc = " * `max-age=30`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::CacheControl;"] # [doc = ""] # [doc = " let cc = CacheControl::new();"] # [doc = " ```"] # [derive (PartialEq , Clone , Debug)] pub struct CacheControl { flags : Flags , max_age : Option < Seconds > , max_stale : Option < Seconds > , min_fresh : Option < Seconds > , s_max_age : Option < Seconds > , }
};
}
