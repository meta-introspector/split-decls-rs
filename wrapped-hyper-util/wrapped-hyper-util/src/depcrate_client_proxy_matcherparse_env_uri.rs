// Generated macro for parse_env_uri (function)
macro_rules! Depcrate_client_proxy_matcherparse_env_uri {
() => {
// Module: crate::client::proxy::matcher
// Provides: {"parse_env_uri"}
// Dependencies: {}
fn parse_env_uri (val : & str) -> Option < Intercept > { use std :: borrow :: Cow ; let uri = val . parse :: < http :: Uri > () . ok () ? ; let mut builder = http :: Uri :: builder () ; let mut is_httpish = false ; let mut auth = Auth :: Empty ; builder = builder . scheme (match uri . scheme () { Some (s) => { if s == & http :: uri :: Scheme :: HTTP || s == & http :: uri :: Scheme :: HTTPS { is_httpish = true ; s . clone () } else if matches ! (s . as_str () , "socks4" | "socks4a" | "socks5" | "socks5h") { s . clone () } else { return None ; } } None => { is_httpish = true ; http :: uri :: Scheme :: HTTP } }) ; let authority = uri . authority () ? ; if let Some ((userinfo , host_port)) = authority . as_str () . split_once ('@') { let (user , pass) = match userinfo . split_once (':') { Some ((user , pass)) => (user , Some (pass)) , None => (userinfo , None) , } ; let user = percent_decode_str (user) . decode_utf8_lossy () ; let pass = pass . map (| pass | percent_decode_str (pass) . decode_utf8_lossy ()) ; if is_httpish { auth = Auth :: Basic (encode_basic_auth (& user , pass . as_deref ())) ; } else { auth = Auth :: Raw (user . into_owned () , pass . map_or_else (String :: new , Cow :: into_owned) ,) ; } builder = builder . authority (host_port) ; } else { builder = builder . authority (authority . clone ()) ; } builder = builder . path_and_query ("/") ; let dst = builder . build () . ok () ? ; Some (Intercept { uri : dst , auth }) }
};
}
