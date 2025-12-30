// Generated macro for path_segment_is_exact_match (function)
macro_rules! Depcrate_deriving_coerce_pointeepath_segment_is_exact_match {
() => {
// Module: crate::deriving::coerce_pointee
// Provides: {"path_segment_is_exact_match"}
// Dependencies: {}
fn path_segment_is_exact_match (path_segments : & [ast :: PathSegment] , syms : & [Symbol]) -> bool { path_segments . iter () . zip (syms) . all (| (segment , & symbol) | segment . ident . name == symbol) }
};
}
