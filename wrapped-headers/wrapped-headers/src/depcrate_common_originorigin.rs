// Generated macro for Origin (struct)
macro_rules! Depcrate_common_originOrigin {
() => {
// Module: crate::common::origin
// Provides: {"Origin"}
// Dependencies: {}
# [doc = " The `Origin` header."] # [doc = ""] # [doc = " The `Origin` header is a version of the `Referer` header that is used for all HTTP fetches and `POST`s whose CORS flag is set."] # [doc = " This header is often used to inform recipients of the security context of where the request was initiated."] # [doc = ""] # [doc = " Following the spec, [https://fetch.spec.whatwg.org/#origin-header][url], the value of this header is composed of"] # [doc = " a String (scheme), Host (host/port)"] # [doc = ""] # [doc = " [url]: https://fetch.spec.whatwg.org/#origin-header"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::Origin;"] # [doc = ""] # [doc = " let origin = Origin::NULL;"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq , Hash)] pub struct Origin (OriginOrNull) ;
};
}
