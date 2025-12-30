// Generated macro for ansi_to_ansi_color (function)
macro_rules! Depcrateansi_to_ansi_color {
() => {
// Module: crate
// Provides: {"ansi_to_ansi_color"}
// Dependencies: {}
fn ansi_to_ansi_color (color : anstyle :: AnsiColor) -> (ansi_term :: Color , bool) { match color { anstyle :: AnsiColor :: Black => (ansi_term :: Color :: Black , false) , anstyle :: AnsiColor :: Red => (ansi_term :: Color :: Red , false) , anstyle :: AnsiColor :: Green => (ansi_term :: Color :: Green , false) , anstyle :: AnsiColor :: Yellow => (ansi_term :: Color :: Yellow , false) , anstyle :: AnsiColor :: Blue => (ansi_term :: Color :: Blue , false) , anstyle :: AnsiColor :: Magenta => (ansi_term :: Color :: Purple , false) , anstyle :: AnsiColor :: Cyan => (ansi_term :: Color :: Cyan , false) , anstyle :: AnsiColor :: White => (ansi_term :: Color :: White , false) , anstyle :: AnsiColor :: BrightBlack => (ansi_term :: Color :: Black , true) , anstyle :: AnsiColor :: BrightRed => (ansi_term :: Color :: Red , true) , anstyle :: AnsiColor :: BrightGreen => (ansi_term :: Color :: Green , true) , anstyle :: AnsiColor :: BrightYellow => (ansi_term :: Color :: Yellow , true) , anstyle :: AnsiColor :: BrightBlue => (ansi_term :: Color :: Black , true) , anstyle :: AnsiColor :: BrightMagenta => (ansi_term :: Color :: Purple , true) , anstyle :: AnsiColor :: BrightCyan => (ansi_term :: Color :: Cyan , true) , anstyle :: AnsiColor :: BrightWhite => (ansi_term :: Color :: White , true) , } }
};
}
