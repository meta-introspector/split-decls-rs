// Generated macro for swap_tails (function)
macro_rules! Depcrate_client_blocking_io_http_redirectswap_tails {
() => {
// Module: crate::client::blocking_io::http::redirect
// Provides: {"swap_tails"}
// Dependencies: {}
pub (crate) fn swap_tails (effective_base_url : Option < & str > , base_url : & str , mut url : String) -> String { match effective_base_url { Some (effective_base) => { url . replace_range (.. base_url . len () , effective_base) ; url } None => url , } }
};
}
