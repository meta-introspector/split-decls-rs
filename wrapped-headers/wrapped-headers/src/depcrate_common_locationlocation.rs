// Generated macro for Location (struct)
macro_rules! Depcrate_common_locationLocation {
() => {
// Module: crate::common::location
// Provides: {"Location"}
// Dependencies: {}
# [doc = " `Location` header, defined in"] # [doc = " [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-7.1.2)"] # [doc = ""] # [doc = " The `Location` header field is used in some responses to refer to a"] # [doc = " specific resource in relation to the response.  The type of"] # [doc = " relationship is defined by the combination of request method and"] # [doc = " status code semantics."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Location = URI-reference"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `/People.html#tim`"] # [doc = " * `http://www.example.net/index.html`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [derive (Clone , Debug , PartialEq)] pub struct Location (HeaderValue) ;
};
}
