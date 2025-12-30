// Generated macro for impl_352 (impl)
macro_rules! Depcrate_proto_ping_pongimpl_352 {
() => {
// Module: crate::proto::ping_pong
// Provides: {"impl_352"}
// Dependencies: {}
impl UserPings { pub (crate) fn send_ping (& self) -> Result < () , Option < proto :: Error > > { let prev = self . 0 . state . compare_exchange (USER_STATE_EMPTY , USER_STATE_PENDING_PING , Ordering :: AcqRel , Ordering :: Acquire ,) . unwrap_or_else (| v | v) ; match prev { USER_STATE_EMPTY => { self . 0 . ping_task . wake () ; Ok (()) } USER_STATE_CLOSED => Err (Some (broken_pipe () . into ())) , _ => { Err (None) } } } pub (crate) fn poll_pong (& self , cx : & mut Context) -> Poll < Result < () , proto :: Error > > { self . 0 . pong_task . register (cx . waker ()) ; let prev = self . 0 . state . compare_exchange (USER_STATE_RECEIVED_PONG , USER_STATE_EMPTY , Ordering :: AcqRel , Ordering :: Acquire ,) . unwrap_or_else (| v | v) ; match prev { USER_STATE_RECEIVED_PONG => Poll :: Ready (Ok (())) , USER_STATE_CLOSED => Poll :: Ready (Err (broken_pipe () . into ())) , _ => Poll :: Pending , } } }
};
}
