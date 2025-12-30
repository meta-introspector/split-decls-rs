// Generated macro for HttpConnector (struct)
macro_rules! Depcrate_client_legacy_connect_httpHttpConnector {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"HttpConnector"}
// Dependencies: {}
# [doc = " A connector for the `http` scheme."] # [doc = ""] # [doc = " Performs DNS resolution in a thread pool, and then connects over TCP."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " Sets the [`HttpInfo`](HttpInfo) value on responses, which includes"] # [doc = " transport information such as the remote socket address used."] # [derive (Clone)] pub struct HttpConnector < R = GaiResolver > { config : Arc < Config > , resolver : R , }
};
}
