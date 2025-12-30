// Generated macro for ETag (struct)
macro_rules! Depcrate_common_etagETag {
() => {
// Module: crate::common::etag
// Provides: {"ETag"}
// Dependencies: {}
# [doc = " `ETag` header, defined in [RFC7232](https://datatracker.ietf.org/doc/html/rfc7232#section-2.3)"] # [doc = ""] # [doc = " The `ETag` header field in a response provides the current entity-tag"] # [doc = " for the selected representation, as determined at the conclusion of"] # [doc = " handling the request.  An entity-tag is an opaque validator for"] # [doc = " differentiating between multiple representations of the same"] # [doc = " resource, regardless of whether those multiple representations are"] # [doc = " due to resource state changes over time, content negotiation"] # [doc = " resulting in multiple representations being valid at the same time,"] # [doc = " or both.  An entity-tag consists of an opaque quoted string, possibly"] # [doc = " prefixed by a weakness indicator."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " ETag       = entity-tag"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `\"xyzzy\"`"] # [doc = " * `W/\"xyzzy\"`"] # [doc = " * `\"\"`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " let etag = \"\\\"xyzzy\\\"\".parse::<headers::ETag>().unwrap();"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq)] pub struct ETag (pub (super) EntityTag) ;
};
}
