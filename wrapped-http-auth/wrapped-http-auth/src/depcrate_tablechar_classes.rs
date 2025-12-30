// Generated macro for char_classes (function)
macro_rules! Depcrate_tablechar_classes {
() => {
// Module: crate::table
// Provides: {"char_classes"}
// Dependencies: {}
pub (crate) fn char_classes (b : u8) -> u8 { * TABLE . get (usize :: from (b)) . unwrap_or (& 0) }
};
}
