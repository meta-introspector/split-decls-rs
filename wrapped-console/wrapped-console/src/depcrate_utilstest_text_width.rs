// Generated macro for test_text_width (function)
macro_rules! Depcrate_utilstest_text_width {
() => {
// Module: crate::utils
// Provides: {"test_text_width"}
// Dependencies: {}
# [test] fn test_text_width () { let s = style ("foo") . red () . on_black () . bold () . force_styling (true) . to_string () ; assert_eq ! (measure_text_width (& s) , if cfg ! (feature = "ansi-parsing") { 3 } else { 21 }) ; let s = style ("🐶 <3") . red () . force_styling (true) . to_string () ; assert_eq ! (measure_text_width (& s) , match (cfg ! (feature = "ansi-parsing") , cfg ! (feature = "unicode-width")) { (true , true) => 5 , (true , false) => 4 , (false , true) => 14 , (false , false) => 13 , }) ; }
};
}
