// Generated macro for SpanMapRef (enum)
macro_rules! Depcrate_span_mapSpanMapRef {
() => {
// Module: crate::span_map
// Provides: {"SpanMapRef"}
// Dependencies: {}
# [derive (Copy , Clone)] pub enum SpanMapRef < 'a > { # [doc = " Spanmap for a macro file"] ExpansionSpanMap (& 'a ExpansionSpanMap) , # [doc = " Spanmap for a real file"] RealSpanMap (& 'a RealSpanMap) , }
};
}
