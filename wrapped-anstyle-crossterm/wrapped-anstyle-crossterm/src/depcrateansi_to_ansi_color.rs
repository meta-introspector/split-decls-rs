// Generated macro for ansi_to_ansi_color (function)
macro_rules! Depcrateansi_to_ansi_color {
() => {
// Module: crate
// Provides: {"ansi_to_ansi_color"}
// Dependencies: {}
fn ansi_to_ansi_color (color : anstyle :: AnsiColor) -> crossterm :: style :: Color { match color { anstyle :: AnsiColor :: Black => crossterm :: style :: Color :: Black , anstyle :: AnsiColor :: Red => crossterm :: style :: Color :: DarkRed , anstyle :: AnsiColor :: Green => crossterm :: style :: Color :: DarkGreen , anstyle :: AnsiColor :: Yellow => crossterm :: style :: Color :: DarkYellow , anstyle :: AnsiColor :: Blue => crossterm :: style :: Color :: DarkBlue , anstyle :: AnsiColor :: Magenta => crossterm :: style :: Color :: DarkMagenta , anstyle :: AnsiColor :: Cyan => crossterm :: style :: Color :: DarkCyan , anstyle :: AnsiColor :: White => crossterm :: style :: Color :: Grey , anstyle :: AnsiColor :: BrightBlack => crossterm :: style :: Color :: DarkGrey , anstyle :: AnsiColor :: BrightRed => crossterm :: style :: Color :: Red , anstyle :: AnsiColor :: BrightGreen => crossterm :: style :: Color :: Green , anstyle :: AnsiColor :: BrightYellow => crossterm :: style :: Color :: Yellow , anstyle :: AnsiColor :: BrightBlue => crossterm :: style :: Color :: Blue , anstyle :: AnsiColor :: BrightMagenta => crossterm :: style :: Color :: Magenta , anstyle :: AnsiColor :: BrightCyan => crossterm :: style :: Color :: Cyan , anstyle :: AnsiColor :: BrightWhite => crossterm :: style :: Color :: White , } }
};
}
