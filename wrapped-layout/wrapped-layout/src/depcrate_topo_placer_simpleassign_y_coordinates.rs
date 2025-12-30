// Generated macro for assign_y_coordinates (function)
macro_rules! Depcrate_topo_placer_simpleassign_y_coordinates {
() => {
// Module: crate::topo::placer::simple
// Provides: {"assign_y_coordinates"}
// Dependencies: {}
# [doc = " Assign the initial Y coordinates."] fn assign_y_coordinates (vg : & mut VisualGraph) { let mut lowest_point = 0. ; for i in 0 .. vg . dag . num_levels () { let current_row = vg . dag . row (i) ; let mut max_height : f64 = 0. ; for idx in current_row . iter () { let height = vg . pos (* idx) . size (true) . y ; max_height = max_height . max (height) ; } let new_center = lowest_point + max_height / 2. ; for idx in current_row . clone () . iter () { let height = vg . pos (* idx) . size (true) . y ; vg . pos_mut (* idx) . align_to_top (new_center - height / 2.) ; } lowest_point += max_height ; } }
};
}
