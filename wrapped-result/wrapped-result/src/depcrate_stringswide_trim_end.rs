// Generated macro for wide_trim_end (function)
macro_rules! Depcrate_stringswide_trim_end {
() => {
// Module: crate::strings
// Provides: {"wide_trim_end"}
// Dependencies: {}
pub fn wide_trim_end (mut wide : & [u16]) -> & [u16] { while let Some (last) = wide . last () { match last { 32 | 9 ..= 13 => wide = & wide [.. wide . len () - 1] , _ => break , } } wide }
};
}
