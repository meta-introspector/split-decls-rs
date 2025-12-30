// Generated macro for Allow (struct)
macro_rules! Depcrate_common_allowAllow {
() => {
// Module: crate::common::allow
// Provides: {"Allow"}
// Dependencies: {}
# [doc = " `Allow` header, defined in [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-7.4.1)"] # [doc = ""] # [doc = " The `Allow` header field lists the set of methods advertised as"] # [doc = " supported by the target resource.  The purpose of this field is"] # [doc = " strictly to inform the recipient of valid request methods associated"] # [doc = " with the resource."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Allow = #method"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `GET, HEAD, PUT`"] # [doc = " * `OPTIONS, GET, PUT, POST, DELETE, HEAD, TRACE, CONNECT, PATCH, fOObAr`"] # [doc = " * ``"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " extern crate http;"] # [doc = " use headers::Allow;"] # [doc = " use http::Method;"] # [doc = ""] # [doc = " let allow = vec![Method::GET, Method::POST]"] # [doc = "     .into_iter()"] # [doc = "     .collect::<Allow>();"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq)] pub struct Allow (FlatCsv) ;
};
}
