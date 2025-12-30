// Generated macro for specified_color (function)
macro_rules! Depcrate_parse_color_tagspecified_color {
() => {
// Module: crate::parse::color_tag
// Provides: {"specified_color"}
// Dependencies: {}
# [doc = " Parses a color which has been prefixed by a specifier like `\"bg:\"` or `\"fg:\"`."] fn specified_color (input : Input < '_ >) -> Result < '_ , Color > { with_failure_message (alt ((map (color_16 (Case :: Lowercase) , Color :: Color16) , map (color_256 (Specified :: True) , | (color , _) | Color :: Color256 (color)) , map (color_rgb (Specified :: True) , | (color , _) | Color :: ColorRgb (color)) ,)) , "Unknown color") (input) }
};
}
