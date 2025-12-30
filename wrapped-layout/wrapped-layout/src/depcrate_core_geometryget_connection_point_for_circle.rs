// Generated macro for get_connection_point_for_circle (function)
macro_rules! Depcrate_core_geometryget_connection_point_for_circle {
() => {
// Module: crate::core::geometry
// Provides: {"get_connection_point_for_circle"}
// Dependencies: {}
# [doc = " This is the implementation of get_connector_location for circle-like shapes."] # [doc = " 'See get_connector_location' for details."] pub fn get_connection_point_for_circle (loc : Point , size : Point , from : Point , force : f64 ,) -> (Point , Point) { let loc = loc ; let dx = from . x - loc . x ; let dy = from . y - loc . y ; let a = size . x / 2. ; let b = size . y / 2. ; let m = dy / dx ; if dx == 0. { let b = b * dy . signum () ; let loc1 = Point :: new (loc . x , loc . y + b) ; return create_vector_of_length (loc1 , from , force) ; } let mut v = ellipse_line_intersection (a , b , m) ; if dx < 0. { v = v . neg () ; } let loc1 = loc . add (v) ; create_vector_of_length (loc1 , from , force) }
};
}
