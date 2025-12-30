// Generated macro for impl_354 (impl)
macro_rules! Depcrate_proto_ping_pongimpl_354 {
() => {
// Module: crate::proto::ping_pong
// Provides: {"impl_354"}
// Dependencies: {}
impl Drop for UserPingsRx { fn drop (& mut self) { self . 0 . state . store (USER_STATE_CLOSED , Ordering :: Release) ; self . 0 . pong_task . wake () ; } }
};
}
