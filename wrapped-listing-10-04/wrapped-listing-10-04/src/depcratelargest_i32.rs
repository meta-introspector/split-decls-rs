// Generated macro for largest_i32 (function)
macro_rules! Depcratelargest_i32 {
() => {
// Module: crate
// Provides: {"largest_i32"}
// Dependencies: {}
fn largest_i32 (list : & [i32]) -> & i32 { let mut largest = & list [0] ; for item in list { if item > largest { largest = item ; } } largest }
};
}
