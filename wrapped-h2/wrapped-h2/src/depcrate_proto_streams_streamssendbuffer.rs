// Generated macro for SendBuffer (struct)
macro_rules! Depcrate_proto_streams_streamsSendBuffer {
() => {
// Module: crate::proto::streams::streams
// Provides: {"SendBuffer"}
// Dependencies: {}
# [doc = " Contains the buffer of frames to be written to the wire."] # [derive (Debug)] struct SendBuffer < B > { inner : Mutex < Buffer < Frame < B > > > , }
};
}
