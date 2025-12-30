// Generated macro for color_styles (function)
macro_rules! Depcratecolor_styles {
() => {
// Module: crate
// Provides: {"color_styles"}
// Dependencies: {}
fn color_styles (styled : & [adapter :: Element] , palette : Palette ,) -> impl Iterator < Item = (String , String) > { let mut colors = std :: collections :: BTreeMap :: new () ; for element in styled { let style = element . style ; if let Some (color) = style . get_fg_color () { colors . insert (color_name (FG_PREFIX , color) , rgb_value (color , palette)) ; } if let Some (color) = style . get_bg_color () { colors . insert (color_name (BG_PREFIX , color) , rgb_value (color , palette)) ; } if let Some (color) = style . get_underline_color () { colors . insert (color_name (UNDERLINE_PREFIX , color) , rgb_value (color , palette) ,) ; } } colors . into_iter () }
};
}
