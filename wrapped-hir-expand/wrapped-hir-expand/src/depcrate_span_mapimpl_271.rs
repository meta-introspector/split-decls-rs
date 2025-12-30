// Generated macro for impl_271 (impl)
macro_rules! Depcrate_span_mapimpl_271 {
() => {
// Module: crate::span_map
// Provides: {"impl_271"}
// Dependencies: {}
impl SpanMapRef < '_ > { pub fn span_for_range (self , range : TextRange) -> Span { match self { Self :: ExpansionSpanMap (span_map) => span_map . span_at (range . start ()) , Self :: RealSpanMap (span_map) => span_map . span_for_range (range) , } } }
};
}
