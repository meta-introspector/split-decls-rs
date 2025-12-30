// Generated macro for ansi_color_to_roff (function)
macro_rules! Depcrateansi_color_to_roff {
() => {
// Module: crate
// Provides: {"ansi_color_to_roff"}
// Dependencies: {}
# [doc = " Map Color and Bright Variants to Roff Color styles"] fn ansi_color_to_roff (color : & AnsiColor) -> & 'static str { match color { AnsiColor :: Black | AnsiColor :: BrightBlack => "black" , AnsiColor :: Red | AnsiColor :: BrightRed => "red" , AnsiColor :: Green | AnsiColor :: BrightGreen => "green" , AnsiColor :: Yellow | AnsiColor :: BrightYellow => "yellow" , AnsiColor :: Blue | AnsiColor :: BrightBlue => "blue" , AnsiColor :: Magenta | AnsiColor :: BrightMagenta => "magenta" , AnsiColor :: Cyan | AnsiColor :: BrightCyan => "cyan" , AnsiColor :: White | AnsiColor :: BrightWhite => "white" , } }
};
}
