// Generated macro for FilterMap (struct)
macro_rules! Depcrate_stream_filter_mapFilterMap {
() => {
// Module: crate::stream::filter_map
// Provides: {"FilterMap"}
// Dependencies: {}
# [doc = " A combinator used to filter the results of a stream and simultaneously map"] # [doc = " them to a different type."] # [doc = ""] # [doc = " This structure is returned by the `Stream::filter_map` method."] pub struct FilterMap < S , F > { stream : S , f : F , }
};
}
