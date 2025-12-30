// Generated macro for ConnectionInner (struct)
macro_rules! Depcrate_proto_connectionConnectionInner {
() => {
// Module: crate::proto::connection
// Provides: {"ConnectionInner"}
// Dependencies: {}
# [derive (Debug)] struct ConnectionInner < P , B : Buf = Bytes > where P : Peer , { # [doc = " Tracks the connection level state transitions."] state : State , # [doc = " An error to report back once complete."] # [doc = ""] # [doc = " This exists separately from State in order to support"] # [doc = " graceful shutdown."] error : Option < frame :: GoAway > , # [doc = " Pending GOAWAY frames to write."] go_away : GoAway , # [doc = " Ping/pong handler"] ping_pong : PingPong , # [doc = " Connection settings"] settings : Settings , # [doc = " Stream state handler"] streams : Streams < B , P > , # [doc = " A `tracing` span tracking the lifetime of the connection."] span : tracing :: Span , # [doc = " Client or server"] _phantom : PhantomData < P > , }
};
}
