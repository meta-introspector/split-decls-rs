// Generated macro for align_self_edges (function)
macro_rules! Depcrate_topo_placer_edge_fixeralign_self_edges {
() => {
// Module: crate::topo::placer::edge_fixer
// Provides: {"align_self_edges"}
// Dependencies: {}
fn align_self_edges (vg : & mut VisualGraph) -> usize { let mut cnt = 0 ; for row_idx in 0 .. vg . dag . num_levels () { let row = vg . dag . row (row_idx) . clone () ; for (i , curr) in row . iter () . enumerate () { if ! vg . is_connector (* curr) { continue ; } let mut found_before = false ; let mut found_after = false ; for pred in vg . dag . predecessors (* curr) { let idx = row . iter () . position (| x | * x == * pred) ; if let Option :: Some (idx) = idx { if idx < i { found_before = true ; } if idx > i { found_after = true ; } } } if found_before { let prev = row [i - 1] ; let prev_pos = vg . pos (prev) ; vg . pos_mut (* curr) . align_to_left (prev_pos . right (true)) ; cnt += 1 ; continue ; } if found_after { let next = row [i + 1] ; let next_pos = vg . pos (next) ; vg . pos_mut (* curr) . align_to_right (next_pos . left (true)) ; cnt += 1 ; continue ; } } } cnt }
};
}
