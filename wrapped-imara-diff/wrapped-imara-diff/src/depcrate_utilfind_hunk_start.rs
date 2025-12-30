// Generated macro for find_hunk_start (function)
macro_rules! Depcrate_utilfind_hunk_start {
() => {
// Module: crate::util
// Provides: {"find_hunk_start"}
// Dependencies: {}
pub fn find_hunk_start (changes : & [bool] , pos : u32) -> u32 { pos - changes [.. pos as usize] . iter () . rev () . take_while (| & & changed | changed) . count () as u32 }
};
}
