// Generated macro for request_bi (function)
macro_rules! Depcrate_clientrequest_bi {
() => {
// Module: crate::client
// Provides: {"request_bi"}
// Dependencies: {}
async fn request_bi (send : quinn :: SendStream , recv : quinn :: RecvStream , upload : u64 , download : u64 , stream_stats : OpenStreamStats ,) -> Result < () > { request (send , upload , download , stream_stats . clone ()) . await ? ; drain_stream (recv , download , stream_stats) . await ? ; Ok (()) }
};
}
