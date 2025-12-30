// Generated macro for SpanMap (enum)
macro_rules! Depcrate_span_mapSpanMap {
() => {
// Module: crate::span_map
// Provides: {"SpanMap"}
// Dependencies: {}
# [doc = " Spanmap for a macro file or a real file"] # [derive (Clone , Debug , PartialEq , Eq)] pub enum SpanMap { # [doc = " Spanmap for a macro file"] ExpansionSpanMap (Arc < ExpansionSpanMap >) , # [doc = " Spanmap for a real file"] RealSpanMap (Arc < RealSpanMap >) , }
};
}
