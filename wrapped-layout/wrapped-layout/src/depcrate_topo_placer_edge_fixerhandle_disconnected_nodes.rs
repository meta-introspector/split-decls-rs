// Generated macro for handle_disconnected_nodes (function)
macro_rules! Depcrate_topo_placer_edge_fixerhandle_disconnected_nodes {
() => {
// Module: crate::topo::placer::edge_fixer
// Provides: {"handle_disconnected_nodes"}
// Dependencies: {}
fn handle_disconnected_nodes (vg : & mut VisualGraph) -> usize { let mut cnt = 0 ; for row_idx in 0 .. vg . dag . num_levels () { let row = vg . dag . row (row_idx) . clone () ; for elem in row . iter () { if ! vg . dag . successors (* elem) . is_empty () || ! vg . dag . predecessors (* elem) . is_empty () { continue ; } let range = compute_bounds_for_node (vg , * elem) ; if range . 0 . is_finite () { vg . pos_mut (* elem) . align_to_left (range . 0 + EPSILON) ; cnt += 1 ; continue ; } if range . 1 . is_finite () { vg . pos_mut (* elem) . align_to_right (range . 1 - EPSILON) ; cnt += 1 ; continue ; } } } cnt }
};
}
