// Generated macro for Referer (struct)
macro_rules! Depcrate_common_refererReferer {
() => {
// Module: crate::common::referer
// Provides: {"Referer"}
// Dependencies: {}
# [doc = " `Referer` header, defined in"] # [doc = " [RFC7231](https://datatracker.ietf.org/doc/html/rfc7231#section-5.5.2)"] # [doc = ""] # [doc = " The `Referer` \\[sic\\] header field allows the user agent to specify a"] # [doc = " URI reference for the resource from which the target URI was obtained"] # [doc = " (i.e., the \"referrer\", though the field name is misspelled).  A user"] # [doc = " agent MUST NOT include the fragment and userinfo components of the"] # [doc = " URI reference, if any, when generating the Referer field value."] # [doc = ""] # [doc = " ## ABNF"] # [doc = ""] # [doc = " ```text"] # [doc = " Referer = absolute-URI / partial-URI"] # [doc = " ```"] # [doc = ""] # [doc = " ## Example values"] # [doc = ""] # [doc = " * `http://www.example.org/hypertext/Overview.html`"] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ```"] # [doc = " use headers::Referer;"] # [doc = ""] # [doc = " let r = Referer::from_static(\"/People.html#tim\");"] # [doc = " ```"] # [derive (Debug , Clone , PartialEq)] pub struct Referer (HeaderValueString) ;
};
}
