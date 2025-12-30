// Generated macro for straighten_edge (function)
macro_rules! Depcrate_topo_placer_edge_fixerstraighten_edge {
() => {
// Module: crate::topo::placer::edge_fixer
// Provides: {"straighten_edge"}
// Dependencies: {}
fn straighten_edge (vg : & mut VisualGraph) -> usize { let mut cnt = 0 ; let mut to_straighten : Vec < NodeHandle > = Vec :: new () ; for row_idx in 1 .. vg . dag . num_levels () - 1 { let row = vg . dag . row (row_idx) ; 'out : for elem in row . iter () { if ! vg . is_connector (* elem) { continue ; } let pred = vg . dag . single_pred (* elem) ; let succ = vg . dag . single_succ (* elem) ; if let Some (pred) = pred { if let Some (succ) = succ { if vg . is_connector (pred) || vg . is_connector (succ) { continue ; } let p1 = vg . pos (pred) . center () ; let p2 = vg . pos (succ) . center () ; let seg = (p1 , p2) ; for elem in row . iter () { let rect = vg . pos (* elem) . bbox (false) ; if segment_rect_intersection (seg , rect) { continue 'out ; } } to_straighten . push (* elem) ; } } } } for elem in to_straighten { let pred = vg . dag . single_pred (elem) . unwrap () ; let succ = vg . dag . single_succ (elem) . unwrap () ; let p1 = vg . pos (pred) . center () ; let p2 = vg . pos (succ) . center () ; let new_pos = p1 . add (p2) . scale (0.5) ; let bounds = compute_bounds_for_node (vg , elem) ; if in_range (bounds , new_pos . x) { vg . pos_mut (elem) . set_x (new_pos . x) ; cnt += 1 ; } } cnt }
};
}
