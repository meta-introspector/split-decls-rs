// Generated macro for xterm_to_ansi (function)
macro_rules! Depcratexterm_to_ansi {
() => {
// Module: crate
// Provides: {"xterm_to_ansi"}
// Dependencies: {}
# [doc = " Lossily convert from the 256-color palette to 4-bit color"] # [doc = ""] # [doc = " As the palette for 4-bit colors is terminal/user defined, a [`palette::Palette`] must be"] # [doc = " provided to match against."] pub const fn xterm_to_ansi (color : anstyle :: Ansi256Color , palette : palette :: Palette ,) -> anstyle :: AnsiColor { match color . 0 { 0 => anstyle :: AnsiColor :: Black , 1 => anstyle :: AnsiColor :: Red , 2 => anstyle :: AnsiColor :: Green , 3 => anstyle :: AnsiColor :: Yellow , 4 => anstyle :: AnsiColor :: Blue , 5 => anstyle :: AnsiColor :: Magenta , 6 => anstyle :: AnsiColor :: Cyan , 7 => anstyle :: AnsiColor :: White , 8 => anstyle :: AnsiColor :: BrightBlack , 9 => anstyle :: AnsiColor :: BrightRed , 10 => anstyle :: AnsiColor :: BrightGreen , 11 => anstyle :: AnsiColor :: BrightYellow , 12 => anstyle :: AnsiColor :: BrightBlue , 13 => anstyle :: AnsiColor :: BrightMagenta , 14 => anstyle :: AnsiColor :: BrightCyan , 15 => anstyle :: AnsiColor :: BrightWhite , _ => { let rgb = XTERM_COLORS [color . 0 as usize] ; palette . find_match (rgb) } } }
};
}
