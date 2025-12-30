// Generated macro for rgb_to_ansi (function)
macro_rules! Depcratergb_to_ansi {
() => {
// Module: crate
// Provides: {"rgb_to_ansi"}
// Dependencies: {}
# [doc = " Lossily convert an RGB value to a 4-bit color"] # [doc = ""] # [doc = " As the palette for 4-bit colors is terminal/user defined, a [`palette::Palette`] must be"] # [doc = " provided to match against."] pub const fn rgb_to_ansi (color : anstyle :: RgbColor , palette : palette :: Palette ,) -> anstyle :: AnsiColor { palette . find_match (color) }
};
}
