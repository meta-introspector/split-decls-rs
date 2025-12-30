// Generated macro for connect (function)
macro_rules! Depcrate_client_blocking_io_httpconnect {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"connect"}
// Dependencies: {}
# [doc = " Connect to the given `url` via HTTP/S using the `desired_version` of the `git` protocol."] # [doc = " If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate."] # [cfg (any (feature = "http-client-curl" , feature = "http-client-reqwest"))] pub fn connect < H : Http + Default > (url : gix_url :: Url , desired_version : Protocol , trace : bool) -> Transport < H > { Transport :: new (url , desired_version , trace) }
};
}
