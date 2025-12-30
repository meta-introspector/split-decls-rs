// Generated macro for HttpInfo (struct)
macro_rules! Depcrate_client_legacy_connect_httpHttpInfo {
() => {
// Module: crate::client::legacy::connect::http
// Provides: {"HttpInfo"}
// Dependencies: {}
# [doc = " Extra information about the transport when an HttpConnector is used."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " # fn doc(res: http::Response<()>) {"] # [doc = " use hyper_util::client::legacy::connect::HttpInfo;"] # [doc = ""] # [doc = " // res = http::Response"] # [doc = " res"] # [doc = "     .extensions()"] # [doc = "     .get::<HttpInfo>()"] # [doc = "     .map(|info| {"] # [doc = "         println!(\"remote addr = {}\", info.remote_addr());"] # [doc = "     });"] # [doc = " # }"] # [doc = " ```"] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " If a different connector is used besides [`HttpConnector`](HttpConnector),"] # [doc = " this value will not exist in the extensions. Consult that specific"] # [doc = " connector to see what \"extra\" information it might provide to responses."] # [derive (Clone , Debug)] pub struct HttpInfo { remote_addr : SocketAddr , local_addr : SocketAddr , }
};
}
