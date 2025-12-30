// Generated macro for Te (struct)
macro_rules! Depcrate_common_teTe {
() => {
// Module: crate::common::te
// Provides: {"Te"}
// Dependencies: {}
# [doc = " `TE` header, defined in"] # [doc = " [RFC7230](https://datatracker.ietf.org/doc/html/rfc7230#section-4.3)"] # [doc = ""] # [doc = " As RFC7230 states, \"The \"TE\" header field in a request indicates what transfer codings,"] # [doc = " besides chunked, the client is willing to accept in response, and"] # [doc = " whether or not the client is willing to accept trailer fields in a"] # [doc = " chunked transfer coding.\""] # [doc = ""] # [doc = " For HTTP/1.1 compliant clients `chunked` transfer codings are assumed to be acceptable and"] # [doc = " so should never appear in this header."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " TE        = \"TE\" \":\" #( t-codings )"] # [doc = " t-codings = \"trailers\" | ( transfer-extension [ accept-params ] )"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `trailers`"] # [doc = " * `trailers, deflate;q=0.5`"] # [doc = " * ``"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [derive (Clone , Debug , PartialEq)] pub struct Te (FlatCsv) ;
};
}
