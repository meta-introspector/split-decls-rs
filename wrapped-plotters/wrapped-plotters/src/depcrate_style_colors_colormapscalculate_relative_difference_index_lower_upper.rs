// Generated macro for calculate_relative_difference_index_lower_upper (function)
macro_rules! Depcrate_style_colors_colormapscalculate_relative_difference_index_lower_upper {
() => {
// Module: crate::style::colors::colormaps
// Provides: {"calculate_relative_difference_index_lower_upper"}
// Dependencies: {}
# [doc (hidden)] pub fn calculate_relative_difference_index_lower_upper < FloatType : num_traits :: Float + num_traits :: FromPrimitive + num_traits :: ToPrimitive , > (h : FloatType , min : FloatType , max : FloatType , n_steps : usize ,) -> (FloatType , usize , usize) { let h = num_traits :: clamp (h , min , max) ; let t = (h - min) / (max - min) ; let approximate_index = t * (FloatType :: from_usize (n_steps) . unwrap () - FloatType :: one ()) . max (FloatType :: zero ()) ; let index_lower = approximate_index . floor () . to_usize () . unwrap () ; let index_upper = approximate_index . ceil () . to_usize () . unwrap () ; let relative_difference = approximate_index . ceil () - approximate_index ; (relative_difference , index_lower , index_upper) }
};
}
