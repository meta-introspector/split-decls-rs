// Generated macro for UserPingsInner (struct)
macro_rules! Depcrate_proto_ping_pongUserPingsInner {
() => {
// Module: crate::proto::ping_pong
// Provides: {"UserPingsInner"}
// Dependencies: {}
# [derive (Debug)] struct UserPingsInner { state : AtomicUsize , # [doc = " Task to wake up the main `Connection`."] ping_task : AtomicWaker , # [doc = " Task to wake up `share::PingPong::poll_pong`."] pong_task : AtomicWaker , }
};
}
