// Generated macro for IfNoneMatch (struct)
macro_rules! Depcrate_common_if_none_matchIfNoneMatch {
() => {
// Module: crate::common::if_none_match
// Provides: {"IfNoneMatch"}
// Dependencies: {}
# [doc = " `If-None-Match` header, defined in"] # [doc = " [RFC7232](https://tools.ietf.org/html/rfc7232#section-3.2)"] # [doc = ""] # [doc = " The `If-None-Match` header field makes the request method conditional"] # [doc = " on a recipient cache or origin server either not having any current"] # [doc = " representation of the target resource, when the field-value is \"*\","] # [doc = " or having a selected representation with an entity-tag that does not"] # [doc = " match any of those listed in the field-value."] # [doc = ""] # [doc = " A recipient MUST use the weak comparison function when comparing"] # [doc = " entity-tags for If-None-Match (Section 2.3.2), since weak entity-tags"] # [doc = " can be used for cache validation even if there have been changes to"] # [doc = " the representation data."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " If-None-Match = \"*\" / 1#entity-tag"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `\"xyzzy\"`"] # [doc = " * `W/\"xyzzy\"`"] # [doc = " * `\"xyzzy\", \"r2d2xxxx\", \"c3piozzzz\"`"] # [doc = " * `W/\"xyzzy\", W/\"r2d2xxxx\", W/\"c3piozzzz\"`"] # [doc = " * `*`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::IfNoneMatch;"] # [doc = ""] # [doc = " let if_none_match = IfNoneMatch::any();"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct IfNoneMatch (EntityTagRange) ;
};
}
