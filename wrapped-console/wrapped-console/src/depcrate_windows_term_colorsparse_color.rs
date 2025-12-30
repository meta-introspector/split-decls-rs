// Generated macro for parse_color (function)
macro_rules! Depcrate_windows_term_colorsparse_color {
() => {
// Module: crate::windows_term::colors
// Provides: {"parse_color"}
// Dependencies: {}
fn parse_color (mut bytes : Bytes < '_ >) -> Option < (Intense , Color , FgBg) > { parse_prefix (& mut bytes) ? ; let fg_bg = FgBg :: new (bytes . next () ?) ? ; let (intense , color) = match bytes . next () ? { b @ b'0' ..= b'7' => (Intense :: No , normal_color_ansi_from_byte (b) ?) , b'8' => { if & [bytes . next () ? , bytes . next () ? , bytes . next () ?] != b";5;" { return None ; } (Intense :: Yes , parse_intense_color_ansi (& mut bytes) ?) } _ => return None , } ; parse_suffix (& mut bytes) ? ; Some ((intense , color , fg_bg)) }
};
}
