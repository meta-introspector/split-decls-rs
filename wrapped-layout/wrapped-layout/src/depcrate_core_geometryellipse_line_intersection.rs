// Generated macro for ellipse_line_intersection (function)
macro_rules! Depcrate_core_geometryellipse_line_intersection {
() => {
// Module: crate::core::geometry
// Provides: {"ellipse_line_intersection"}
// Dependencies: {}
# [doc = " \\returns the intersection point for a line with slope \\p m with an ellipse"] # [doc = " with the formula. 1 = (x^2 / a^2) + (y^2 / b^2)."] # [doc = " Replace Y with the line equation and isolate x and solve to get the"] # [doc = " intersection point with the ellipse."] # [doc = " Notice that a line has two intersection points with a circle, so users need"] # [doc = " to figure out which of the two values (+X, +Y) or (-X, -Y) is relevant."] pub fn ellipse_line_intersection (a : f64 , b : f64 , m : f64) -> Point { let x : f64 = ((a * a * b * b) / (b * b + a * a * m * m)) . sqrt () ; let y : f64 = m * x ; Point :: new (x , y) }
};
}
