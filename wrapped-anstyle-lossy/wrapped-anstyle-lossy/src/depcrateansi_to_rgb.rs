// Generated macro for ansi_to_rgb (function)
macro_rules! Depcrateansi_to_rgb {
() => {
// Module: crate
// Provides: {"ansi_to_rgb"}
// Dependencies: {}
# [doc = " Lossily convert from 4-bit color to RGB"] # [doc = ""] # [doc = " As the palette for 4-bit colors is terminal/user defined, a [`palette::Palette`] must be"] # [doc = " provided to match against."] pub const fn ansi_to_rgb (color : anstyle :: AnsiColor , palette : palette :: Palette ,) -> anstyle :: RgbColor { palette . rgb_from_ansi (color) }
};
}
