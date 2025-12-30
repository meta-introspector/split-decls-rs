// Generated macro for preprocess_invert_style (function)
macro_rules! Depcratepreprocess_invert_style {
() => {
// Module: crate
// Provides: {"preprocess_invert_style"}
// Dependencies: {}
fn preprocess_invert_style (elements : & mut [adapter :: Element] , bg_color : anstyle :: Color , fg_color : anstyle :: Color ,) { for element in elements { let style = & mut element . style ; if style . get_effects () . contains (anstyle :: Effects :: INVERT) { * style = style . fg_color (Some (style . get_bg_color () . unwrap_or (bg_color))) . bg_color (Some (style . get_fg_color () . unwrap_or (fg_color))) . effects (style . get_effects () . remove (anstyle :: Effects :: INVERT)) ; } } }
};
}
