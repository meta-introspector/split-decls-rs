// Generated macro for extract_domain (function)
macro_rules! Depcrate_client_legacy_clientextract_domain {
() => {
// Module: crate::client::legacy::client
// Provides: {"extract_domain"}
// Dependencies: {}
fn extract_domain (uri : & mut Uri , is_http_connect : bool) -> Result < PoolKey , Error > { let uri_clone = uri . clone () ; match (uri_clone . scheme () , uri_clone . authority ()) { (Some (scheme) , Some (auth)) => Ok ((scheme . clone () , auth . clone ())) , (None , Some (auth)) if is_http_connect => { let scheme = match auth . port_u16 () { Some (443) => { set_scheme (uri , Scheme :: HTTPS) ; Scheme :: HTTPS } _ => { set_scheme (uri , Scheme :: HTTP) ; Scheme :: HTTP } } ; Ok ((scheme , auth . clone ())) } _ => { debug ! ("Client requires absolute-form URIs, received: {:?}" , uri) ; Err (e ! (UserAbsoluteUriRequired)) } } }
};
}
