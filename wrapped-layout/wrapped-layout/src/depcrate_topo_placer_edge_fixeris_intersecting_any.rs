// Generated macro for is_intersecting_any (function)
macro_rules! Depcrate_topo_placer_edge_fixeris_intersecting_any {
() => {
// Module: crate::topo::placer::edge_fixer
// Provides: {"is_intersecting_any"}
// Dependencies: {}
fn is_intersecting_any (segs : & [Segment] , rects : & [Rect]) -> bool { for seg in segs { for rec in rects { if segment_rect_intersection (* seg , * rec) { return true ; } } } false }
};
}
