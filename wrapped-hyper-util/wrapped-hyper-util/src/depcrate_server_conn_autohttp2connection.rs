// Generated macro for Http2Connection (type)
macro_rules! Depcrate_server_conn_autoHttp2Connection {
() => {
// Module: crate::server::conn::auto
// Provides: {"Http2Connection"}
// Dependencies: {}
# [cfg (not (feature = "http2"))] type Http2Connection < I , S , E > = (PhantomData < I > , PhantomData < S > , PhantomData < E >) ;
};
}
