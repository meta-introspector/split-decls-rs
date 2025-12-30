// Generated macro for connect_http (function)
macro_rules! Depcrate_client_blocking_io_httpconnect_http {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"connect_http"}
// Dependencies: {}
# [doc = " Connect to the given `url` via HTTP/S using the `desired_version` of the `git` protocol, with `http` as implementation."] # [doc = " If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate."] # [cfg (all (feature = "http-client" , not (feature = "http-client-curl")))] pub fn connect_http < H : Http > (http : H , url : gix_url :: Url , desired_version : Protocol , trace : bool) -> Transport < H > { Transport :: new_http (http , url , desired_version , trace) }
};
}
