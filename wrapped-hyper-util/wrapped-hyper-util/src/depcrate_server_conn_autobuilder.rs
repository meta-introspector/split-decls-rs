// Generated macro for Builder (struct)
macro_rules! Depcrate_server_conn_autoBuilder {
() => {
// Module: crate::server::conn::auto
// Provides: {"Builder"}
// Dependencies: {}
# [doc = " Http1 or Http2 connection builder."] # [derive (Clone , Debug)] pub struct Builder < E > { # [cfg (feature = "http1")] http1 : http1 :: Builder , # [cfg (feature = "http2")] http2 : http2 :: Builder < E > , # [cfg (any (feature = "http1" , feature = "http2"))] version : Option < Version > , # [cfg (not (feature = "http2"))] _executor : E , }
};
}
