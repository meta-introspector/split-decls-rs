// Generated macro for UserAgent (struct)
macro_rules! Depcrate_common_user_agentUserAgent {
() => {
// Module: crate::common::user_agent
// Provides: {"UserAgent"}
// Dependencies: {}
# [doc = " `User-Agent` header, defined in"] # [doc = " [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-5.5.3)"] # [doc = ""] # [doc = " The `User-Agent` header field contains information about the user"] # [doc = " agent originating the request, which is often used by servers to help"] # [doc = " identify the scope of reported interoperability problems, to work"] # [doc = " around or tailor responses to avoid particular user agent"] # [doc = " limitations, and for analytics regarding browser or operating system"] # [doc = " use.  A user agent SHOULD send a User-Agent field in each request"] # [doc = " unless specifically configured not to do so."] # [doc = ""] # [doc = " # ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " User-Agent = product *( RWS ( product / comment ) )"] # [doc = " product         = token [\"/\" product-version]"] # [doc = " product-version = token"] # [doc = " ```"] # [doc = ""] # [doc = " # Example values"] # [doc = ""] # [doc = " * `CERN-LineMode/2.15 libwww/2.17b3`"] # [doc = " * `Bunnies`"] # [doc = ""] # [doc = " # Notes"] # [doc = ""] # [doc = " * The parser does not split the value"] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::UserAgent;"] # [doc = ""] # [doc = " let ua = UserAgent::from_static(\"hyper/0.12.2\");"] # [doc = " ```"] # [derive (Clone , Debug , PartialEq , Eq , PartialOrd , Ord , Hash)] pub struct UserAgent (HeaderValueString) ;
};
}
