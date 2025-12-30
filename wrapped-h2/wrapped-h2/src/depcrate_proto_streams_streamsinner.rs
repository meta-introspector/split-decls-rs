// Generated macro for Inner (struct)
macro_rules! Depcrate_proto_streams_streamsInner {
() => {
// Module: crate::proto::streams::streams
// Provides: {"Inner"}
// Dependencies: {}
# [doc = " Fields needed to manage state related to managing the set of streams. This"] # [doc = " is mostly split out to make ownership happy."] # [doc = ""] # [doc = " TODO: better name"] # [derive (Debug)] struct Inner { # [doc = " Tracks send & recv stream concurrency."] counts : Counts , # [doc = " Connection level state and performs actions on streams"] actions : Actions , # [doc = " Stores stream state"] store : Store , # [doc = " The number of stream refs to this shared state."] refs : usize , }
};
}
