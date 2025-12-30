// Generated macro for fetch_manifest (function)
macro_rules! Depcratefetch_manifest {
() => {
// Module: crate
// Provides: {"fetch_manifest"}
// Dependencies: {}
fn fetch_manifest (config : & Stage0Config , channel : & str , date : Option < & str > ,) -> Result < Manifest , Error > { let url = if let Some (date) = date { format ! ("{}/dist/{}/channel-rust-{}.toml" , config . dist_server , date , channel) } else { format ! ("{}/dist/channel-rust-{}.toml" , config . dist_server , channel) } ; let response = http_get (& url) ? ; let response = String :: from_utf8 (response) ? ; Ok (toml :: from_str (& response) ?) }
};
}
