// Generated macro for hyper_request (struct)
macro_rules! Depcrate_ffi_http_typeshyper_request {
() => {
// Module: crate::ffi::http_types
// Provides: {"hyper_request"}
// Dependencies: {}
# [doc = " An HTTP request."] # [doc = ""] # [doc = " Once you've finished constructing a request, you can send it with"] # [doc = " `hyper_clientconn_send`."] # [doc = ""] # [doc = " Methods:"] # [doc = ""] # [doc = " - hyper_request_new:              Construct a new HTTP request."] # [doc = " - hyper_request_headers:          Gets a mutable reference to the HTTP headers of this request"] # [doc = " - hyper_request_set_body:         Set the body of the request."] # [doc = " - hyper_request_set_method:       Set the HTTP Method of the request."] # [doc = " - hyper_request_set_uri:          Set the URI of the request."] # [doc = " - hyper_request_set_uri_parts:    Set the URI of the request with separate scheme, authority, and path/query strings."] # [doc = " - hyper_request_set_version:      Set the preferred HTTP version of the request."] # [doc = " - hyper_request_on_informational: Set an informational (1xx) response callback."] # [doc = " - hyper_request_free:             Free an HTTP request."] pub struct hyper_request (pub (super) Request < IncomingBody >) ;
};
}
