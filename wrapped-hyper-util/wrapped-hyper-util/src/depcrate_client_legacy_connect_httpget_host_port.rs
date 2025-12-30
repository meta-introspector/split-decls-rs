// Generated macro for get_host_port (function)
macro_rules! Depcrate_client_legacy_connect_httpget_host_port {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"get_host_port"}
// Dependencies: {}
fn get_host_port < 'u > (config : & Config , dst : & 'u Uri) -> Result < (& 'u str , u16) , ConnectError > { trace ! ("Http::connect; scheme={:?}, host={:?}, port={:?}" , dst . scheme () , dst . host () , dst . port () ,) ; if config . enforce_http { if dst . scheme () != Some (& Scheme :: HTTP) { return Err (ConnectError { msg : INVALID_NOT_HTTP , addr : None , cause : None , }) ; } } else if dst . scheme () . is_none () { return Err (ConnectError { msg : INVALID_MISSING_SCHEME , addr : None , cause : None , }) ; } let host = match dst . host () { Some (s) => s , None => { return Err (ConnectError { msg : INVALID_MISSING_HOST , addr : None , cause : None , }) ; } } ; let port = match dst . port () { Some (port) => port . as_u16 () , None => { if dst . scheme () == Some (& Scheme :: HTTPS) { 443 } else { 80 } } } ; Ok ((host , port)) }
};
}
