// Generated macro for is_bright (function)
macro_rules! Depcrateis_bright {
() => {
// Module: crate
// Provides: {"is_bright"}
// Dependencies: {}
# [doc = " Check if [`Color`] is an [`AnsiColor::Bright*`][AnsiColor] variant"] fn is_bright (fg_color : & Color) -> bool { if let Color :: Ansi (color) = fg_color { matches ! (color , AnsiColor :: BrightRed | AnsiColor :: BrightBlue | AnsiColor :: BrightBlack | AnsiColor :: BrightCyan | AnsiColor :: BrightGreen | AnsiColor :: BrightWhite | AnsiColor :: BrightYellow | AnsiColor :: BrightMagenta) } else { false } }
};
}
