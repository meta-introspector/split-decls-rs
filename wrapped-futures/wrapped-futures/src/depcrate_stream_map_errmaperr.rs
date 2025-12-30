// Generated macro for MapErr (struct)
macro_rules! Depcrate_stream_map_errMapErr {
() => {
// Module: crate::stream::map_err
// Provides: {"MapErr"}
// Dependencies: {}
# [doc = " A stream combinator which will change the error type of a stream from one"] # [doc = " type to another."] # [doc = ""] # [doc = " This is produced by the `Stream::map_err` method."] pub struct MapErr < S , F > { stream : S , f : F , }
};
}
