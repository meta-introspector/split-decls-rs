// Generated macro for Merge (struct)
macro_rules! Depcrate_stream_mergeMerge {
() => {
// Module: crate::stream::merge
// Provides: {"Merge"}
// Dependencies: {}
# [doc = " An adapter for merging the output of two streams."] # [doc = ""] # [doc = " The merged stream produces items from one or both of the underlying"] # [doc = " streams as they become available. Errors, however, are not merged: you"] # [doc = " get at most one error at a time."] pub struct Merge < S1 , S2 : Stream > { stream1 : Fuse < S1 > , stream2 : Fuse < S2 > , queued_error : Option < S2 :: Error > , }
};
}
