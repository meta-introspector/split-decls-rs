// Generated macro for Streams (struct)
macro_rules! Depcrate_proto_streams_streamsStreams {
() => {
// Module: crate::proto::streams::streams
// Provides: {"Streams"}
// Dependencies: {}
# [derive (Debug)] pub (crate) struct Streams < B , P > where P : Peer , { # [doc = " Holds most of the connection and stream related state for processing"] # [doc = " HTTP/2 frames associated with streams."] inner : Arc < Mutex < Inner > > , # [doc = " This is the queue of frames to be written to the wire. This is split out"] # [doc = " to avoid requiring a `B` generic on all public API types even if `B` is"] # [doc = " not technically required."] # [doc = ""] # [doc = " Currently, splitting this out requires a second `Arc` + `Mutex`."] # [doc = " However, it should be possible to avoid this duplication with a little"] # [doc = " bit of unsafe code. This optimization has been postponed until it has"] # [doc = " been shown to be necessary."] send_buffer : Arc < SendBuffer < B > > , _p : :: std :: marker :: PhantomData < P > , }
};
}
