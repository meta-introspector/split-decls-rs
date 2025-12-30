// Generated macro for Filter (struct)
macro_rules! Depcrate_stream_filterFilter {
() => {
// Module: crate::stream::filter
// Provides: {"Filter"}
// Dependencies: {}
# [doc = " A stream combinator used to filter the results of a stream and only yield"] # [doc = " some values."] # [doc = ""] # [doc = " This structure is produced by the `Stream::filter` method."] pub struct Filter < S , F > { stream : S , f : F , }
};
}
