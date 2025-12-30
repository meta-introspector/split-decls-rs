// Generated macro for move_text_up (function)
macro_rules! Depcrate_topo_placer_move_between_rowsmove_text_up {
() => {
// Module: crate::topo::placer::move_between_rows
// Provides: {"move_text_up"}
// Dependencies: {}
fn move_text_up (vg : & mut VisualGraph) -> usize { let mut prev_row_size = get_row_width (vg , 0) ; let mut cnt = 0 ; for i in 1 .. vg . dag . num_levels () { let row = vg . dag . row (i) . clone () ; let mut curr_row_size = get_row_width (vg , i) ; for elem in row . iter () { if ! vg . is_connector (* elem) { continue ; } if vg . dag . predecessors (* elem) . len () != 1 { continue ; } let pred = vg . dag . predecessors (* elem) [0] ; if ! vg . is_connector (pred) { continue ; } let pred_node_size = vg . pos (pred) . size (true) . x ; let curr_node_size = vg . pos (* elem) . size (true) . x ; if prev_row_size + curr_node_size < curr_row_size { if move_label (vg , * elem , pred) { curr_row_size -= curr_node_size ; prev_row_size += curr_node_size ; cnt += 1 ; continue ; } } if prev_row_size > curr_row_size + pred_node_size { if move_label (vg , pred , * elem) { curr_row_size += pred_node_size ; prev_row_size -= pred_node_size ; cnt += 1 ; continue ; } } } } cnt }
};
}
