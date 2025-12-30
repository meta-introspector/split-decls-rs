// Generated macro for Collect (struct)
macro_rules! Depcrate_stream_collectCollect {
() => {
// Module: crate::stream::collect
// Provides: {"Collect"}
// Dependencies: {}
# [doc = " A future which collects all of the values of a stream into a vector."] # [doc = ""] # [doc = " This future is created by the `Stream::collect` method."] pub struct Collect < S > where S : Stream { stream : S , items : Vec < S :: Item > , }
};
}
