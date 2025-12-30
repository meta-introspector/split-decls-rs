// Generated macro for calc_indent (function)
macro_rules! Depcrate_scannerscalc_indent {
() => {
// Module: crate::scanners
// Provides: {"calc_indent"}
// Dependencies: {}
pub (crate) fn calc_indent (text : & [u8] , max : usize) -> (usize , usize) { let mut spaces = 0 ; let mut offset = 0 ; for (i , & b) in text . iter () . enumerate () { offset = i ; match b { b' ' => { spaces += 1 ; if spaces == max { break ; } } b'\t' => { let new_spaces = spaces + 4 - (spaces & 3) ; if new_spaces > max { break ; } spaces = new_spaces ; } _ => break , } } (offset , spaces) }
};
}
