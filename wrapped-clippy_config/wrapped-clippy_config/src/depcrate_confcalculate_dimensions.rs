// Generated macro for calculate_dimensions (function)
macro_rules! Depcrate_confcalculate_dimensions {
() => {
// Module: crate::conf
// Provides: {"calculate_dimensions"}
// Dependencies: {}
fn calculate_dimensions (fields : & [& str]) -> (usize , Vec < usize >) { let columns = env :: var ("CLIPPY_TERMINAL_WIDTH") . ok () . and_then (| s | < usize as FromStr > :: from_str (& s) . ok ()) . map_or (1 , | terminal_width | { let max_field_width = fields . iter () . map (| field | field . len ()) . max () . unwrap () ; cmp :: max (1 , terminal_width / (SEPARATOR_WIDTH + max_field_width)) }) ; let rows = fields . len () . div_ceil (columns) ; let column_widths = (0 .. columns) . map (| column | { if column < columns - 1 { (0 .. rows) . map (| row | { let index = column * rows + row ; let field = fields . get (index) . copied () . unwrap_or_default () ; field . len () }) . max () . unwrap () } else { 0 } }) . collect :: < Vec < _ > > () ; (rows , column_widths) }
};
}
