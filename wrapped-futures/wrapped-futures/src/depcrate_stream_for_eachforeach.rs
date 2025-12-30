// Generated macro for ForEach (struct)
macro_rules! Depcrate_stream_for_eachForEach {
() => {
// Module: crate::stream::for_each
// Provides: {"ForEach"}
// Dependencies: {}
# [doc = " A stream combinator which executes a unit closure over each item on a"] # [doc = " stream."] # [doc = ""] # [doc = " This structure is returned by the `Stream::for_each` method."] pub struct ForEach < S , F > { stream : S , f : F , }
};
}
