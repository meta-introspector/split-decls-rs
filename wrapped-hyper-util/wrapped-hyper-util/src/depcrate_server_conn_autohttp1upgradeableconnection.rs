// Generated macro for Http1UpgradeableConnection (type)
macro_rules! Depcrate_server_conn_autoHttp1UpgradeableConnection {
() => {
// Module: crate::server::conn::auto
// Provides: {"Http1UpgradeableConnection"}
// Dependencies: {}
# [cfg (not (feature = "http1"))] type Http1UpgradeableConnection < I , S > = (PhantomData < I > , PhantomData < S >) ;
};
}
