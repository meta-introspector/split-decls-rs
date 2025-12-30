// Generated macro for align_to_left (function)
macro_rules! Depcrate_topo_placer_simplealign_to_left {
() => {
// Module: crate::topo::placer::simple
// Provides: {"align_to_left"}
// Dependencies: {}
# [doc = " Move the whole graph all the way to the left."] pub (crate) fn align_to_left (vg : & mut VisualGraph) { let mut first_x : f64 = 10000. ; for elem in vg . iter_nodes () { let loc = vg . pos (elem) . bbox (true) . 0 . x ; first_x = first_x . min (loc) ; } for elem in vg . iter_nodes () { vg . pos_mut (elem) . translate (Point :: new (- first_x , 0.)) ; } }
};
}
