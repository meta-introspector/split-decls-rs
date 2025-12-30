// Generated macro for next_u8 (function)
macro_rules! Depcrate_helpersnext_u8 {
() => {
// Module: crate::helpers
// Provides: {"next_u8"}
// Dependencies: {}
pub (crate) fn next_u8 < F > (mut index : u8 , condition : F) -> u8 where F : Fn (u8) -> bool , { loop { if condition (index) { return index ; } index += 1 ; } }
};
}
