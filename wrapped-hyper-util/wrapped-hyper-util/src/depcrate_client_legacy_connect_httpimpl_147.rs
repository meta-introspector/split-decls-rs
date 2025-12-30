// Generated macro for impl_147 (impl)
macro_rules! Depcrate_client_legacy_connect_httpimpl_147 {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"impl_147"}
// Dependencies: {}
impl < R : Resolve > Future for HttpConnecting < R > { type Output = ConnectResult ; fn poll (self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Self :: Output > { self . project () . fut . poll (cx) } }
};
}
