// Generated macro for StreamRef (struct)
macro_rules! Depcrate_proto_streams_streamsStreamRef {
() => {
// Module: crate::proto::streams::streams
// Provides: {"StreamRef"}
// Dependencies: {}
# [doc = " Reference to the stream state"] # [derive (Debug)] pub (crate) struct StreamRef < B > { opaque : OpaqueStreamRef , send_buffer : Arc < SendBuffer < B > > , }
};
}
