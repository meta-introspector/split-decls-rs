// Generated macro for Server (struct)
macro_rules! Depcrate_common_serverServer {
() => {
// Module: crate::common::server
// Provides: {"Server"}
// Dependencies: {}
# [doc = " `Server` header, defined in [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-7.4.2)"] # [doc = ""] # [doc = " The `Server` header field contains information about the software"] # [doc = " used by the origin server to handle the request, which is often used"] # [doc = " by clients to help identify the scope of reported interoperability"] # [doc = " problems, to work around or tailor requests to avoid particular"] # [doc = " server limitations, and for analytics regarding server or operating"] # [doc = " system use.  An origin server MAY generate a Server field in its"] # [doc = " responses."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Server = product *( RWS ( product / comment ) )"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = " * `CERN/3.0 libwww/2.17`"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::Server;"] # [doc = ""] # [doc = " let server = Server::from_static(\"hyper/0.12.2\");"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct Server (HeaderValueString) ;
};
}
