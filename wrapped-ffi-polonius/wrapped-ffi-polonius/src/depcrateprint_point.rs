// Generated macro for print_point (function)
macro_rules! Depcrateprint_point {
() => {
// Module: crate
// Provides: {"print_point"}
// Dependencies: {}
fn print_point (point : GccrsAtom) { let val : usize = point . into () ; let mid = val % 2 == 1 ; let bb = val >> 16 ; let hide_left_most_bit = val >> 1 ; let stmt = hide_left_most_bit & 0x7FFF ; eprint ! ("{}(bb{}[{}])" , if mid { "Mid" } else { "Start" } , bb , stmt) ; }
};
}
