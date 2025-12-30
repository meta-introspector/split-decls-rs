// Generated macro for tests (module)
macro_rules! Depcrate_options_configtests {
() => {
// Module: crate::options::config
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn parse_none_color_from_string () { for case in & ["" , "none" , "None"] { assert_eq ! (color_from_str (case) , None) ; } } # [test] fn parse_default_color_from_string () { for case in & ["default" , "Default"] { assert_eq ! (color_from_str (case) , Some (Color :: Default)) ; } } # [test] fn parse_fixed_color_from_string () { for case in & ["black" , "Black"] { assert_eq ! (color_from_str (case) , Some (Color :: Black)) ; } } # [test] fn parse_long_hex_color_from_string () { for case in & ["#ff00ff" , "#FF00FF"] { assert_eq ! (color_from_str (case) , Some (Color :: Rgb (255 , 0 , 255))) ; } } # [test] fn parse_short_hex_color_from_string () { for case in ["#f0f" , "#F0F"] . iter () { assert_eq ! (color_from_str (case) , Some (Color :: Rgb (255 , 0 , 255))) ; } } # [test] fn parse_color_code_from_string () { for (s , c) in & [("10" , 10) , ("01" , 1)] { assert_eq ! (color_from_str (s) , Some (Color :: Fixed (* c))) ; } } }
};
}
