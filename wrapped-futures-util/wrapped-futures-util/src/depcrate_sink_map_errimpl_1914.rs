// Generated macro for impl_1914 (impl)
macro_rules! Depcrate_sink_map_errimpl_1914 {
() => {
// Module: crate::sink::map_err
// Provides: {"impl_1914"}
// Dependencies: {}
impl < S : Stream , F > Stream for SinkMapErr < S , F > { type Item = S :: Item ; delegate_stream ! (sink) ; }
};
}
