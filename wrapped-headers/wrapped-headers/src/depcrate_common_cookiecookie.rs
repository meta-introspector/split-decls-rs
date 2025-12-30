// Generated macro for Cookie (struct)
macro_rules! Depcrate_common_cookieCookie {
() => {
// Module: crate::common::cookie
// Provides: {"Cookie"}
// Dependencies: {}
# [doc = " `Cookie` header, defined in [RFC6265](https://datatracker.ietf.org/doc/html/rfc6265#section-5.4)"] # [doc = ""] # [doc = " If the user agent does attach a Cookie header field to an HTTP"] # [doc = " request, the user agent must send the cookie-string"] # [doc = " as the value of the header field."] # [doc = ""] # [doc = " When the user agent generates an HTTP request, the user agent MUST NOT"] # [doc = " attach more than one Cookie header field."] # [doc = ""] # [doc = " # Example values"] # [doc = " * `SID=31d4d96e407aad42`"] # [doc = " * `SID=31d4d96e407aad42; lang=en-US`"] # [doc = ""] # [derive (Clone , Debug)] pub struct Cookie (FlatCsv < SemiColon >) ;
};
}
