// Generated macro for impl_894 (impl)
macro_rules! Depcrate_shareimpl_894 {
() => {
// Module: crate::share
// Provides: {"impl_894"}
// Dependencies: {}
impl PingPong { pub (crate) fn new (inner : proto :: UserPings) -> Self { PingPong { inner } } # [doc = " Send a PING frame and wait for the peer to send the pong."] pub async fn ping (& mut self , ping : Ping) -> Result < Pong , crate :: Error > { self . send_ping (ping) ? ; crate :: poll_fn (| cx | self . poll_pong (cx)) . await } # [doc (hidden)] pub fn send_ping (& mut self , ping : Ping) -> Result < () , crate :: Error > { let _ = ping ; self . inner . send_ping () . map_err (| err | match err { Some (err) => err . into () , None => UserError :: SendPingWhilePending . into () , }) } # [doc (hidden)] pub fn poll_pong (& mut self , cx : & mut Context) -> Poll < Result < Pong , crate :: Error > > { ready ! (self . inner . poll_pong (cx)) ? ; Poll :: Ready (Ok (Pong { _p : () })) } }
};
}
