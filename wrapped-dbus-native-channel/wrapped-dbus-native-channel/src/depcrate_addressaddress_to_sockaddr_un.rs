// Generated macro for address_to_sockaddr_un (function)
macro_rules! Depcrate_addressaddress_to_sockaddr_un {
() => {
// Module: crate::address
// Provides: {"address_to_sockaddr_un"}
// Dependencies: {}
pub fn address_to_sockaddr_un (s : & str) -> Result < libc :: sockaddr_un , Box < dyn std :: error :: Error > > { if ! s . starts_with ("unix:") { Err ("Address is not a unix socket") ? } ; for pair in s ["unix:" . len () ..] . split (',') { let mut kv = pair . splitn (2 , "=") ; if let Some (key) = kv . next () { if let Some (value) = kv . next () { if key == "path" { return make_sockaddr_un (0 , value) ; } if key == "abstract" { return make_sockaddr_un (1 , value) ; } } } } Err (format ! ("unsupported address type: {}" , s)) ? }
};
}
