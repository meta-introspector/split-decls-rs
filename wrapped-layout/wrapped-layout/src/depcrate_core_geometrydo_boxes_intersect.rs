// Generated macro for do_boxes_intersect (function)
macro_rules! Depcrate_core_geometrydo_boxes_intersect {
() => {
// Module: crate::core::geometry
// Provides: {"do_boxes_intersect"}
// Dependencies: {}
# [doc = " \\return True if the boxes (defined by the bounding box) intersect."] pub fn do_boxes_intersect (p1 : (Point , Point) , p2 : (Point , Point)) -> bool { let overlap_x = smaller_than_or_equal_to_f64 (p2 . 0 . x , p1 . 1 . x) && smaller_than_or_equal_to_f64 (p1 . 0 . x , p2 . 1 . x) ; let overlap_y = smaller_than_or_equal_to_f64 (p2 . 0 . y , p1 . 1 . y) && smaller_than_or_equal_to_f64 (p1 . 0 . y , p2 . 1 . y) ; overlap_x && overlap_y }
};
}
