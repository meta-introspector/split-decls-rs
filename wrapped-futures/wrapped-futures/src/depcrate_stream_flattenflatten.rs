// Generated macro for Flatten (struct)
macro_rules! Depcrate_stream_flattenFlatten {
() => {
// Module: crate::stream::flatten
// Provides: {"Flatten"}
// Dependencies: {}
# [doc = " A combinator used to flatten a stream-of-streams into one long stream of"] # [doc = " elements."] # [doc = ""] # [doc = " This combinator is created by the `Stream::flatten` method."] pub struct Flatten < S > where S : Stream , { stream : S , next : Option < S :: Item > , }
};
}
