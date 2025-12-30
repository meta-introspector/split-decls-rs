// Generated macro for get_connection_point_for_box (function)
macro_rules! Depcrate_core_geometryget_connection_point_for_box {
() => {
// Module: crate::core::geometry
// Provides: {"get_connection_point_for_box"}
// Dependencies: {}
# [doc = " This is the implementation of get_connector_location for box-like shapes."] # [doc = " 'See get_connector_location' for details."] pub fn get_connection_point_for_box (loc : Point , size : Point , from : Point , force : f64 ,) -> (Point , Point) { let mut loc = loc ; let mut size = size ; if from . x > loc . x + size . x / 2. { size . x /= 2. ; loc . x += size . x / 2. ; } else if from . x < loc . x - size . x / 2. { size . x /= 2. ; loc . x -= size . x / 2. ; } let dx = loc . x - from . x ; let dy = loc . y - from . y ; let mut box_x = size . x / 2. ; let mut box_y = size . y / 2. ; if dx == 0. { if dy > 0. { let loc = Point :: new (loc . x , loc . y - box_y) ; return create_vector_of_length (loc , from , force) ; } else { let loc = Point :: new (loc . x , loc . y + box_y) ; return create_vector_of_length (loc , from , force) ; } } let slope_from = dy / dx ; let mut gain_y = box_x * slope_from ; if gain_y . abs () < box_y { if dx > 0. { box_x = - box_x ; gain_y = - gain_y ; } let con = Point :: new (loc . x + box_x , loc . y + gain_y) ; return create_vector_of_length (con , from , force) ; } let mut gain_x = box_y / slope_from ; if dy > 0. { box_y = - box_y ; gain_x = - gain_x ; } let con = Point :: new (loc . x + gain_x , loc . y + box_y) ; create_vector_of_length (con , from , force) }
};
}
