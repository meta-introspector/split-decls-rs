// Generated macro for find_next_change (function)
macro_rules! Depcrate_utilfind_next_change {
() => {
// Module: crate::util
// Provides: {"find_next_change"}
// Dependencies: {}
pub fn find_next_change (changes : & [bool] , pos : u32) -> Option < u32 > { changes [pos as usize ..] . iter () . position (| & changed | changed) . map (| off | off as u32) }
};
}
