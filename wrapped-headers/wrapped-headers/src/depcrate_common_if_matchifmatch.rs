// Generated macro for IfMatch (struct)
macro_rules! Depcrate_common_if_matchIfMatch {
() => {
// Module: crate::common::if_match
// Provides: {"IfMatch"}
// Dependencies: {}
# [doc = " `If-Match` header, defined in"] # [doc = " [RFC7232](https://tools.ietf.org/html/rfc7232#section-3.1)"] # [doc = ""] # [doc = " The `If-Match` header field makes the request method conditional on"] # [doc = " the recipient origin server either having at least one current"] # [doc = " representation of the target resource, when the field-value is \"*\","] # [doc = " or having a current representation of the target resource that has an"] # [doc = " entity-tag matching a member of the list of entity-tags provided in"] # [doc = " the field-value."] # [doc = ""] # [doc = " An origin server MUST use the strong comparison function when"] # [doc = " comparing entity-tags for `If-Match`, since the client"] # [doc = " intends this precondition to prevent the method from being applied if"] # [doc = " there have been any changes to the representation data."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " If-Match = \"*\" / 1#entity-tag"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `\"xyzzy\"`"] # [doc = " * \"xyzzy\", \"r2d2xxxx\", \"c3piozzzz\""] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::IfMatch;"] # [doc = ""] # [doc = " let if_match = IfMatch::any();"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct IfMatch (EntityTagRange) ;
};
}
