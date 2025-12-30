// Generated macro for get_size_for_str (function)
macro_rules! Depcrate_core_geometryget_size_for_str {
() => {
// Module: crate::core::geometry
// Provides: {"get_size_for_str"}
// Dependencies: {}
# [doc = " Estimate the bounding box of some rendered text."] pub fn get_size_for_str (label : & str , font_size : usize) -> Point { let max_line_len = if ! label . is_empty () { label . lines () . map (| x | x . chars () . count ()) . max () . unwrap () } else { 0 } ; let ts = (max_line_len . max (1) , label . lines () . count () . max (1)) ; Point :: new (ts . 0 as f64 , ts . 1 as f64) . scale (font_size as f64) }
};
}
