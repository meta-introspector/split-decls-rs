// Generated macro for generate_curve_for_elements (function)
macro_rules! Depcrate_std_shapes_rendergenerate_curve_for_elements {
() => {
// Module: crate::std_shapes::render
// Provides: {"generate_curve_for_elements"}
// Dependencies: {}
pub fn generate_curve_for_elements (elements : & [Element] , arrow : & Arrow , force : f64 ,) -> Vec < (Point , Point) > { let mut path : Vec < (Point , Point) > = Vec :: new () ; let to_loc = elements [1] . position () . center () ; let from_con = elements [0] . get_connector_location (to_loc , force , & arrow . src_port) ; let mut prev_exit_loc = from_con . 0 ; path . push ((from_con . 0 , from_con . 1)) ; for i in 1 .. elements . len () { let to_con ; let is_last : bool = i == elements . len () - 1 ; if is_last { let to = & elements [i] ; to_con = to . get_connector_location (prev_exit_loc , force , & arrow . dst_port ,) ; prev_exit_loc = to_con . 0 ; } else { let center = & elements [i] ; let to = & elements [i + 1] ; let to_loc = to . position () . center () ; to_con = center . get_passthrough_path (prev_exit_loc , to_loc , force) ; prev_exit_loc = to_con . 0 ; } path . push ((to_con . 1 , to_con . 0)) ; } path }
};
}
