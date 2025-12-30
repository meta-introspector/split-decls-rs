// Generated macro for DynConnection (struct)
macro_rules! Depcrate_proto_connectionDynConnection {
() => {
// Module: crate::proto::connection
// Provides: {"DynConnection"}
// Dependencies: {}
struct DynConnection < 'a , B : Buf = Bytes > { state : & 'a mut State , go_away : & 'a mut GoAway , streams : DynStreams < 'a , B > , error : & 'a mut Option < frame :: GoAway > , ping_pong : & 'a mut PingPong , }
};
}
