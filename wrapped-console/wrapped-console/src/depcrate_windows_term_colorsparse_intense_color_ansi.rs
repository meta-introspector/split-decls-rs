// Generated macro for parse_intense_color_ansi (function)
macro_rules! Depcrate_windows_term_colorsparse_intense_color_ansi {
() => {
// Module: crate::windows_term::colors
// Provides: {"parse_intense_color_ansi"}
// Dependencies: {}
fn parse_intense_color_ansi (bytes : & mut Bytes < '_ >) -> Option < Color > { let color = match bytes . next () ? { b'8' => Color :: Black , b'9' => Color :: Red , b'1' => match bytes . next () ? { b'0' => Color :: Green , b'1' => Color :: Yellow , b'2' => Color :: Blue , b'3' => Color :: Magenta , b'4' => Color :: Cyan , b'5' => Color :: White , _ => return None , } , _ => return None , } ; Some (color) }
};
}
