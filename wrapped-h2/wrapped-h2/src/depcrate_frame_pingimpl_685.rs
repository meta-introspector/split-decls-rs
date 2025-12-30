// Generated macro for impl_685 (impl)
macro_rules! Depcrate_frame_pingimpl_685 {
() => {
// Module: crate::frame::ping
// Provides: {"impl_685"}
// Dependencies: {}
impl Ping { # [cfg (feature = "unstable")] pub const SHUTDOWN : Payload = SHUTDOWN_PAYLOAD ; # [cfg (not (feature = "unstable"))] pub (crate) const SHUTDOWN : Payload = SHUTDOWN_PAYLOAD ; # [cfg (feature = "unstable")] pub const USER : Payload = USER_PAYLOAD ; # [cfg (not (feature = "unstable"))] pub (crate) const USER : Payload = USER_PAYLOAD ; pub fn new (payload : Payload) -> Ping { Ping { ack : false , payload , } } pub fn pong (payload : Payload) -> Ping { Ping { ack : true , payload } } pub fn is_ack (& self) -> bool { self . ack } pub fn payload (& self) -> & Payload { & self . payload } pub fn into_payload (self) -> Payload { self . payload } # [doc = " Builds a `Ping` frame from a raw frame."] pub fn load (head : Head , bytes : & [u8]) -> Result < Ping , Error > { debug_assert_eq ! (head . kind () , crate :: frame :: Kind :: Ping) ; if ! head . stream_id () . is_zero () { return Err (Error :: InvalidStreamId) ; } if bytes . len () != 8 { return Err (Error :: BadFrameSize) ; } let mut payload = [0 ; 8] ; payload . copy_from_slice (bytes) ; let ack = head . flag () & ACK_FLAG != 0 ; Ok (Ping { ack , payload }) } pub fn encode < B : BufMut > (& self , dst : & mut B) { let sz = self . payload . len () ; tracing :: trace ! ("encoding PING; ack={} len={}" , self . ack , sz) ; let flags = if self . ack { ACK_FLAG } else { 0 } ; let head = Head :: new (Kind :: Ping , flags , StreamId :: zero ()) ; head . encode (sz , dst) ; dst . put_slice (& self . payload) ; } }
};
}
