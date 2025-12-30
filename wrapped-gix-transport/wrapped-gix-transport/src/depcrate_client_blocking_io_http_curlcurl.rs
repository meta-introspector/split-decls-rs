// Generated macro for Curl (struct)
macro_rules! Depcrate_client_blocking_io_http_curlCurl {
() => {
// Module: crate::client::blocking_io::http::curl
// Provides: {"Curl"}
// Dependencies: {}
# [doc = " A utility to abstract interactions with curl handles."] pub struct Curl { req : SyncSender < remote :: Request > , res : Receiver < remote :: Response > , handle : Option < thread :: JoinHandle < Result < () , Error > > > , config : http :: Options , }
};
}
