// Generated macro for compute_bounds_for_node (function)
macro_rules! Depcrate_topo_placer_edge_fixercompute_bounds_for_node {
() => {
// Module: crate::topo::placer::edge_fixer
// Provides: {"compute_bounds_for_node"}
// Dependencies: {}
# [doc = " Return the leftmost and rightmost x coordinate that are taken by another"] # [doc = " shape."] fn compute_bounds_for_node (vg : & VisualGraph , node : NodeHandle) -> (f64 , f64) { let level = vg . dag . level (node) ; let row = vg . dag . row (level) ; assert ! (! row . is_empty () , "Empty Row!") ; let pos = vg . pos (node) ; let idx = row . iter () . position (| x | * x == node) . unwrap () ; let mut leftmost = f64 :: NEG_INFINITY ; if idx > 0 { let prev = row [idx - 1] ; leftmost = vg . pos (prev) . right (true) ; } let mut rightmost = f64 :: INFINITY ; if idx < row . len () - 1 { let next = row [idx + 1] ; rightmost = vg . pos (next) . left (true) ; } let loc = pos . center () ; assert ! (loc . x >= leftmost) ; assert ! (loc . x <= rightmost) ; (leftmost , rightmost) }
};
}
