// Generated macro for impl_353 (impl)
macro_rules! Depcrate_proto_ping_pongimpl_353 {
() => {
// Module: crate::proto::ping_pong
// Provides: {"impl_353"}
// Dependencies: {}
impl UserPingsRx { fn receive_pong (& self) -> bool { let prev = self . 0 . state . compare_exchange (USER_STATE_PENDING_PONG , USER_STATE_RECEIVED_PONG , Ordering :: AcqRel , Ordering :: Acquire ,) . unwrap_or_else (| v | v) ; if prev == USER_STATE_PENDING_PONG { self . 0 . pong_task . wake () ; true } else { false } } }
};
}
