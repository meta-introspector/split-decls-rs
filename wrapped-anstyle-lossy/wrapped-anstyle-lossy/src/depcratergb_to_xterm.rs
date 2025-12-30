// Generated macro for rgb_to_xterm (function)
macro_rules! Depcratergb_to_xterm {
() => {
// Module: crate
// Provides: {"rgb_to_xterm"}
// Dependencies: {}
# [doc = " Lossily convert an RGB value to the 256-color palette"] pub const fn rgb_to_xterm (color : anstyle :: RgbColor) -> anstyle :: Ansi256Color { let index = find_xterm_match (color) ; anstyle :: Ansi256Color (index as u8) }
};
}
