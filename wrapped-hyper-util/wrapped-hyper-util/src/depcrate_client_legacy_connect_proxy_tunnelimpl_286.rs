// Generated macro for impl_286 (impl)
macro_rules! Depcrate_client_legacy_connect_proxy_tunnelimpl_286 {
() => {
// Module: crate::client::legacy::connect::proxy::tunnel
// Provides: {"impl_286"}
// Dependencies: {}
impl < F , T , E > Future for Tunneling < F , T > where F : Future < Output = Result < T , E > > , { type Output = Result < T , TunnelError > ; fn poll (self : Pin < & mut Self > , cx : & mut task :: Context < '_ >) -> Poll < Self :: Output > { self . project () . fut . poll (cx) } }
};
}
