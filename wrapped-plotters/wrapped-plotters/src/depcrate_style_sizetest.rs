// Generated macro for test (module)
macro_rules! Depcrate_style_sizetest {
() => {
// Module: crate::style::size
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn test_relative_size () { let size = (10) . percent_height () ; assert_eq ! (size . in_pixels (& (100 , 200)) , 20) ; let size = (10) . percent_width () ; assert_eq ! (size . in_pixels (& (100 , 200)) , 10) ; let size = (- 10) . percent_width () ; assert_eq ! (size . in_pixels (& (100 , 200)) , - 10) ; let size = (10) . percent_width () . min (30) ; assert_eq ! (size . in_pixels (& (100 , 200)) , 30) ; assert_eq ! (size . in_pixels (& (400 , 200)) , 40) ; let size = (10) . percent () ; assert_eq ! (size . in_pixels (& (100 , 200)) , 10) ; assert_eq ! (size . in_pixels (& (400 , 200)) , 20) ; } }
};
}
