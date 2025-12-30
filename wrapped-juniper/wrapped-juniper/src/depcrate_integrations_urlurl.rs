// Generated macro for Url (type)
macro_rules! Depcrate_integrations_urlUrl {
() => {
// Module: crate::integrations::url
// Provides: {"Url"}
// Dependencies: {}
# [doc = " [Standard URL][0] format as specified in [RFC 3986]."] # [doc = ""] # [doc = " [`URL` scalar][1] compliant."] # [doc = ""] # [doc = " See also [`url::Url`][2] for details."] # [doc = ""] # [doc = " [0]: http://url.spec.whatwg.org"] # [doc = " [1]: https://graphql-scalars.dev/docs/scalars/url"] # [doc = " [2]: https://docs.rs/url/*/url/struct.Url.html"] # [doc = " [RFC 3986]: https://datatracker.ietf.org/doc/html/rfc3986"] # [graphql_scalar] # [graphql (name = "URL" , with = url_scalar , to_output_with = Url :: as_str , parse_token (String) , specified_by_url = "https://graphql-scalars.dev/docs/scalars/url" ,)] type Url = url :: Url ;
};
}
