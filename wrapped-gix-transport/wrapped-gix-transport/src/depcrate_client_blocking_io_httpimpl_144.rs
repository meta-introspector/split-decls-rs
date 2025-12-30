// Generated macro for impl_144 (impl)
macro_rules! Depcrate_client_blocking_io_httpimpl_144 {
() => {
// Module: crate::client::blocking_io::http
// Provides: {"impl_144"}
// Dependencies: {}
impl < H : Http > Transport < H > { # [doc = " Create a new instance with `http` as implementation to communicate to `url` using the given `desired_version`."] # [doc = " Note that we will always fallback to other versions as supported by the server."] # [doc = " If `trace` is `true`, all packetlines received or sent will be passed to the facilities of the `gix-trace` crate."] pub fn new_http (http : H , url : gix_url :: Url , desired_version : Protocol , trace : bool) -> Self { let identity = url . user () . zip (url . password ()) . map (| (user , pass) | gix_sec :: identity :: Account { username : user . to_string () , password : pass . to_string () , oauth_refresh_token : None , }) ; Transport { url : url . to_bstring () . to_string () , user_agent_header : concat ! ("User-Agent: git/oxide-" , env ! ("CARGO_PKG_VERSION")) , desired_version , actual_version : Default :: default () , service : None , http , line_provider : None , identity , trace , } } }
};
}
