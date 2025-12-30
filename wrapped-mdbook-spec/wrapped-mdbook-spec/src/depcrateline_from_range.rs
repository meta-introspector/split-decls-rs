// Generated macro for line_from_range (function)
macro_rules! Depcrateline_from_range {
() => {
// Module: crate
// Provides: {"line_from_range"}
// Dependencies: {}
fn line_from_range < 'a > (contents : & 'a str , range : & Range < usize >) -> & 'a str { assert ! (range . start < contents . len ()) ; let mut start_index = 0 ; for line in contents . lines () { let end_index = start_index + line . len () ; if range . start >= start_index && range . start <= end_index { return line ; } start_index = end_index + 1 ; } panic ! ("did not find line {range:?} in contents") ; }
};
}
