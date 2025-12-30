// Generated macro for get_passthrough_path_invisible (function)
macro_rules! Depcrate_core_geometryget_passthrough_path_invisible {
() => {
// Module: crate::core::geometry
// Provides: {"get_passthrough_path_invisible"}
// Dependencies: {}
pub fn get_passthrough_path_invisible (_size : Point , center : Point , from : Point , to : Point , force : f64 ,) -> (Point , Point) { let ar = center . sub (from) ; let rb = to . sub (center) ; let a_outgoing_edge = normalize_scale_vector (ar . neg () , force) ; let b_outgoing_edge = normalize_scale_vector (rb . neg () , force) ; let sum = a_outgoing_edge . add (b_outgoing_edge) ; if sum . length () < 1. { let edge = a_outgoing_edge . rotate (90_f64 . to_radians ()) ; return (center , edge . add (center)) ; } let total = ar . length () + rb . length () ; let mut a_ratio = ar . length () / total ; if center . x == to . x || center . y == to . y { a_ratio = 1. ; } else if center . x == from . x || center . y == from . y { a_ratio = 0. ; } let res = interpolate (a_outgoing_edge , b_outgoing_edge , 1. - a_ratio) ; (center , res . add (center)) }
};
}
