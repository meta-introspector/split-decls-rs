// Generated macro for normal_color_ansi_from_byte (function)
macro_rules! Depcrate_windows_term_colorsnormal_color_ansi_from_byte {
() => {
// Module: crate::windows_term::colors
// Provides: {"normal_color_ansi_from_byte"}
// Dependencies: {}
fn normal_color_ansi_from_byte (b : u8) -> Option < Color > { let color = match b { b'0' => Color :: Black , b'1' => Color :: Red , b'2' => Color :: Green , b'3' => Color :: Yellow , b'4' => Color :: Blue , b'5' => Color :: Magenta , b'6' => Color :: Cyan , b'7' => Color :: White , _ => return None , } ; Some (color) }
};
}
