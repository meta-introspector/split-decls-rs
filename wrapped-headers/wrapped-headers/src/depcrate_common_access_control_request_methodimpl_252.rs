// Generated macro for impl_252 (impl)
macro_rules! Depcrate_common_access_control_request_methodimpl_252 {
() => {
// Module: crate::common::access_control_request_method
// Provides: {"impl_252"}
// Dependencies: {}
impl Header for AccessControlRequestMethod { fn name () -> & 'static HeaderName { & :: http :: header :: ACCESS_CONTROL_REQUEST_METHOD } fn decode < 'i , I : Iterator < Item = & 'i HeaderValue > > (values : & mut I) -> Result < Self , Error > { values . next () . and_then (| value | Method :: from_bytes (value . as_bytes ()) . ok ()) . map (AccessControlRequestMethod) . ok_or_else (Error :: invalid) } fn encode < E : Extend < HeaderValue > > (& self , values : & mut E) { let s = match self . 0 { Method :: GET => "GET" , Method :: POST => "POST" , Method :: PUT => "PUT" , Method :: DELETE => "DELETE" , _ => { let val = HeaderValue :: from_str (self . 0 . as_ref ()) . expect ("Methods are also valid HeaderValues") ; values . extend (:: std :: iter :: once (val)) ; return ; } } ; values . extend (:: std :: iter :: once (HeaderValue :: from_static (s))) ; } }
};
}
