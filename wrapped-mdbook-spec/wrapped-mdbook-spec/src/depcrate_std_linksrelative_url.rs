// Generated macro for relative_url (function)
macro_rules! Depcrate_std_linksrelative_url {
() => {
// Module: crate::std_links
// Provides: {"relative_url"}
// Dependencies: {}
# [doc = " Converts a URL to doc.rust-lang.org to be relative."] fn relative_url (url : & str , chapter : & Chapter) -> String { if std :: env :: var ("SPEC_RELATIVE") . as_deref () != Ok ("0") { let Some (url_start) = DOC_URL . shortest_match (url) else { bug ! ("expected rustdoc URL to start with {DOC_URL:?}, got {url}") ; } ; let url_path = & url [url_start ..] ; let num_dots = chapter . path . as_ref () . unwrap () . components () . count () ; let dots = vec ! [".." ; num_dots] . join ("/") ; format ! ("{dots}{url_path}") } else { url . to_string () } }
};
}
