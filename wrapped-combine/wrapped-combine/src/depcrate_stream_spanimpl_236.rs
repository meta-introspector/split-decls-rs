// Generated macro for impl_236 (impl)
macro_rules! Depcrate_stream_spanimpl_236 {
() => {
// Module: crate::stream::span
// Provides: {"impl_236"}
// Dependencies: {}
impl < P > Span < P > { pub fn map < Q > (self , mut f : impl FnMut (P) -> Q) -> Span < Q > { Span { start : f (self . start) , end : f (self . end) , } } }
};
}
