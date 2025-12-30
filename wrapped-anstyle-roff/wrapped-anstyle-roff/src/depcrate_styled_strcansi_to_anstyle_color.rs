// Generated macro for cansi_to_anstyle_color (function)
macro_rules! Depcrate_styled_strcansi_to_anstyle_color {
() => {
// Module: crate::styled_str
// Provides: {"cansi_to_anstyle_color"}
// Dependencies: {}
fn cansi_to_anstyle_color (color : Option < Color >) -> Option < AColor > { match color { Some (Color :: Black) => Some (AColor :: Ansi (AnsiColor :: Black)) , Some (Color :: Red) => Some (AColor :: Ansi (AnsiColor :: Red)) , Some (Color :: Green) => Some (AColor :: Ansi (AnsiColor :: Green)) , Some (Color :: Yellow) => Some (AColor :: Ansi (AnsiColor :: Yellow)) , Some (Color :: Blue) => Some (AColor :: Ansi (AnsiColor :: Blue)) , Some (Color :: Magenta) => Some (AColor :: Ansi (AnsiColor :: Magenta)) , Some (Color :: Cyan) => Some (AColor :: Ansi (AnsiColor :: Cyan)) , Some (Color :: White) => Some (AColor :: Ansi (AnsiColor :: White)) , Some (Color :: BrightBlack) => Some (AColor :: Ansi (AnsiColor :: BrightBlack)) , Some (Color :: BrightRed) => Some (AColor :: Ansi (AnsiColor :: BrightRed)) , Some (Color :: BrightGreen) => Some (AColor :: Ansi (AnsiColor :: BrightGreen)) , Some (Color :: BrightYellow) => Some (AColor :: Ansi (AnsiColor :: BrightYellow)) , Some (Color :: BrightBlue) => Some (AColor :: Ansi (AnsiColor :: BrightBlue)) , Some (Color :: BrightMagenta) => Some (AColor :: Ansi (AnsiColor :: BrightMagenta)) , Some (Color :: BrightCyan) => Some (AColor :: Ansi (AnsiColor :: BrightCyan)) , Some (Color :: BrightWhite) => Some (AColor :: Ansi (AnsiColor :: BrightWhite)) , None => None , } }
};
}
