// Generated macro for largest_char (function)
macro_rules! Depcratelargest_char {
() => {
// Module: crate
// Provides: {"largest_char"}
// Dependencies: {}
fn largest_char (list : & [char]) -> & char { let mut largest = & list [0] ; for item in list { if item > largest { largest = item ; } } largest }
};
}
