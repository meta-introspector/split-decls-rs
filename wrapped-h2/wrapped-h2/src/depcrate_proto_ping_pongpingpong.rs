// Generated macro for PingPong (struct)
macro_rules! Depcrate_proto_ping_pongPingPong {
() => {
// Module: crate::proto::ping_pong
// Provides: {"PingPong"}
// Dependencies: {}
# [doc = " Acknowledges ping requests from the remote."] # [derive (Debug)] pub (crate) struct PingPong { pending_ping : Option < PendingPing > , pending_pong : Option < PingPayload > , user_pings : Option < UserPingsRx > , }
};
}
