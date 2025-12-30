// Generated macro for make_partial_axis (function)
macro_rules! Depcrate_coord_ranged1d_combinators_partial_axismake_partial_axis {
() => {
// Module: crate::coord::ranged1d::combinators::partial_axis
// Provides: {"make_partial_axis"}
// Dependencies: {}
# [doc = " Make a partial axis based on the percentage of visible portion."] # [doc = " We can use `into_partial_axis` to create a partial axis range specification."] # [doc = " But sometimes, we want to directly specify the percentage visible to the user."] # [doc = ""] # [doc = " - `axis_range`: The range specification"] # [doc = " - `part`: The visible part of the axis. Each value is from [0.0, 1.0]"] # [doc = " - **returns**: The partial axis created from the input, or `None` when not possible"] pub fn make_partial_axis < T > (axis_range : Range < T > , part : Range < f64 > ,) -> Option < PartialAxis < < Range < T > as AsRangedCoord > :: CoordDescType > > where Range < T > : AsRangedCoord , T : num_traits :: NumCast + Clone , { let left : f64 = num_traits :: cast (axis_range . start . clone ()) ? ; let right : f64 = num_traits :: cast (axis_range . end . clone ()) ? ; let full_range_size = (right - left) / (part . end - part . start) ; let full_left = left - full_range_size * part . start ; let full_right = right + full_range_size * (1.0 - part . end) ; let full_range : Range < T > = num_traits :: cast (full_left) ? .. num_traits :: cast (full_right) ? ; let axis_range : < Range < T > as AsRangedCoord > :: CoordDescType = axis_range . into () ; Some (PartialAxis (full_range . into () , axis_range . range ())) }
};
}
