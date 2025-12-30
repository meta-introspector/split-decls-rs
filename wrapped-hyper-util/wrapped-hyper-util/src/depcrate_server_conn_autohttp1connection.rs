// Generated macro for Http1Connection (type)
macro_rules! Depcrate_server_conn_autoHttp1Connection {
() => {
// Module: crate::server::conn::auto
// Provides: {"Http1Connection"}
// Dependencies: {}
# [cfg (not (feature = "http1"))] type Http1Connection < I , S > = (PhantomData < I > , PhantomData < S >) ;
};
}
