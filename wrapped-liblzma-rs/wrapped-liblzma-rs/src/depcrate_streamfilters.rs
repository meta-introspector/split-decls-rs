// Generated macro for Filters (struct)
macro_rules! Depcrate_streamFilters {
() => {
// Module: crate::stream
// Provides: {"Filters"}
// Dependencies: {}
# [doc = " A custom chain of filters to configure an encoding stream."] pub struct Filters { inner : Vec < liblzma_sys :: lzma_filter > , lzma_opts : LinkedList < liblzma_sys :: lzma_options_lzma > , }
};
}
