// Generated macro for Buffered (struct)
macro_rules! Depcrate_stream_bufferedBuffered {
() => {
// Module: crate::stream::buffered
// Provides: {"Buffered"}
// Dependencies: {}
# [doc = " An adaptor for a stream of futures to execute the futures concurrently, if"] # [doc = " possible."] # [doc = ""] # [doc = " This adaptor will buffer up a list of pending futures, and then return their"] # [doc = " results in the order that they're finished. This is created by the"] # [doc = " `Stream::buffered` method."] pub struct Buffered < S > where S : Stream , S :: Item : IntoFuture , { stream : Fuse < S > , futures : Vec < Option < Collapsed < < S :: Item as IntoFuture > :: Future > > > , }
};
}
