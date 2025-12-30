// Generated macro for request_uni (function)
macro_rules! Depcrate_clientrequest_uni {
() => {
// Module: crate::client
// Provides: {"request_uni"}
// Dependencies: {}
async fn request_uni (send : quinn :: SendStream , conn : quinn :: Connection , upload : u64 , download : u64 , stream_stats : OpenStreamStats ,) -> Result < () > { request (send , upload , download , stream_stats . clone ()) . await ? ; let recv = conn . accept_uni () . await ? ; drain_stream (recv , download , stream_stats) . await ? ; Ok (()) }
};
}
