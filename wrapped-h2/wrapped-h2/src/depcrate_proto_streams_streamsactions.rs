// Generated macro for Actions (struct)
macro_rules! Depcrate_proto_streams_streamsActions {
() => {
// Module: crate::proto::streams::streams
// Provides: {"Actions"}
// Dependencies: {}
# [derive (Debug)] struct Actions { # [doc = " Manages state transitions initiated by receiving frames"] recv : Recv , # [doc = " Manages state transitions initiated by sending frames"] send : Send , # [doc = " Task that calls `poll_complete`."] task : Option < Waker > , # [doc = " If the connection errors, a copy is kept for any StreamRefs."] conn_error : Option < proto :: Error > , }
};
}
