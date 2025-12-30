// Generated macro for Term (struct)
macro_rules! DepcrateTerm {
() => {
// Module: crate
// Provides: {"Term"}
// Dependencies: {}
# [doc = " Define the terminal-like settings for rendering output"] # [derive (Copy , Clone , Debug)] pub struct Term { palette : Palette , fg_color : anstyle :: Color , bg_color : anstyle :: Color , background : bool , font_family : & 'static str , min_width_px : usize , padding_px : usize , }
};
}
