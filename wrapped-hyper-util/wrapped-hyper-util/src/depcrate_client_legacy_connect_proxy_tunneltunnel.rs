// Generated macro for Tunnel (struct)
macro_rules! Depcrate_client_legacy_connect_proxy_tunnelTunnel {
() => {
// Module: crate::client::legacy::connect::proxy::tunnel
// Provides: {"Tunnel"}
// Dependencies: {}
# [doc = " Tunnel Proxy via HTTP CONNECT"] # [doc = ""] # [doc = " This is a connector that can be used by the `legacy::Client`. It wraps"] # [doc = " another connector, and after getting an underlying connection, it creates"] # [doc = " an HTTP CONNECT tunnel over it."] # [derive (Debug , Clone)] pub struct Tunnel < C > { headers : Headers , inner : C , proxy_dst : Uri , }
};
}
