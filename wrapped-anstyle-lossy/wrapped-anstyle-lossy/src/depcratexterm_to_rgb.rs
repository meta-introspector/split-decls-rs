// Generated macro for xterm_to_rgb (function)
macro_rules! Depcratexterm_to_rgb {
() => {
// Module: crate
// Provides: {"xterm_to_rgb"}
// Dependencies: {}
# [doc = " Lossily convert from 256-color to RGB"] # [doc = ""] # [doc = " As 256-color palette is a superset of 4-bit colors and since the palette for 4-bit colors is"] # [doc = " terminal/user defined, a [`palette::Palette`] must be provided to match against."] pub const fn xterm_to_rgb (color : anstyle :: Ansi256Color , palette : palette :: Palette ,) -> anstyle :: RgbColor { match palette . rgb_from_index (color . 0) { Some (rgb) => rgb , None => XTERM_COLORS [color . 0 as usize] , } }
};
}
