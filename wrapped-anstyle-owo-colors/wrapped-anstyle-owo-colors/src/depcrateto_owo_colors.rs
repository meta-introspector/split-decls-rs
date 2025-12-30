// Generated macro for to_owo_colors (function)
macro_rules! Depcrateto_owo_colors {
() => {
// Module: crate
// Provides: {"to_owo_colors"}
// Dependencies: {}
# [doc = " Adapt generic colors to [`owo_colors`]"] pub fn to_owo_colors (color : anstyle :: Color) -> owo_colors :: DynColors { match color { anstyle :: Color :: Ansi (ansi) => owo_colors :: DynColors :: Ansi (ansi_to_owo_colors_color (ansi)) , anstyle :: Color :: Ansi256 (xterm) => { owo_colors :: DynColors :: Xterm (xterm_to_owo_colors_color (xterm)) } anstyle :: Color :: Rgb (rgb) => { let (r , g , b) = rgb_to_owo_colors_color (rgb) ; owo_colors :: DynColors :: Rgb (r , g , b) } } }
};
}
