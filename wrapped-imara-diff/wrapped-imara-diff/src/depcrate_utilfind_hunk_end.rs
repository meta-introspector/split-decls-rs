// Generated macro for find_hunk_end (function)
macro_rules! Depcrate_utilfind_hunk_end {
() => {
// Module: crate::util
// Provides: {"find_hunk_end"}
// Dependencies: {}
pub fn find_hunk_end (changes : & [bool] , pos : u32) -> u32 { pos + changes [pos as usize ..] . iter () . take_while (| & & changed | changed) . count () as u32 }
};
}
