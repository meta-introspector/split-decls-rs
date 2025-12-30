// Generated macro for measure_text_width (function)
macro_rules! Depcrate_utilsmeasure_text_width {
() => {
// Module: crate::utils
// Provides: {"measure_text_width"}
// Dependencies: {}
# [doc = " Measure the width of a string in terminal characters."] pub fn measure_text_width (s : & str) -> usize { # [cfg (feature = "ansi-parsing")] { AnsiCodeIterator :: new (s) . filter_map (| (s , is_ansi) | match is_ansi { false => Some (str_width (s)) , true => None , }) . sum () } # [cfg (not (feature = "ansi-parsing"))] { str_width (s) } }
};
}
