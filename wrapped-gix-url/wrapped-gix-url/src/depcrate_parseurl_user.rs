// Generated macro for url_user (function)
macro_rules! Depcrate_parseurl_user {
() => {
// Module: crate::parse
// Provides: {"url_user"}
// Dependencies: {}
fn url_user (url : & crate :: simple_url :: ParsedUrl < '_ > , kind : UrlKind) -> Result < Option < String > , Error > { if url . username . is_empty () && url . password . is_none () { Ok (None) } else { Ok (Some (percent_decoded_utf8 (url . username , kind) ?)) } }
};
}
