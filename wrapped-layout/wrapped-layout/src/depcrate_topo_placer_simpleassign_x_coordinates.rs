// Generated macro for assign_x_coordinates (function)
macro_rules! Depcrate_topo_placer_simpleassign_x_coordinates {
() => {
// Module: crate::topo::placer::simple
// Provides: {"assign_x_coordinates"}
// Dependencies: {}
# [doc = " Assign the initial x coordinates based on the natural ordering in the"] # [doc = " rank."] fn assign_x_coordinates (vg : & mut VisualGraph) { for i in 0 .. vg . dag . num_levels () { let current_row = vg . dag . row (i) ; let mut rightmost_point = 0. ; for idx in current_row . clone () . iter () { let pos = vg . pos_mut (* idx) ; pos . align_to_left (rightmost_point + EPSILON) ; rightmost_point = pos . bbox (true) . 1 . x + EPSILON ; } } }
};
}
