// Generated macro for largest (function)
macro_rules! Depcratelargest {
() => {
// Module: crate
// Provides: {"largest"}
// Dependencies: {}
fn largest < T > (list : & [T]) -> & T { let mut largest = & list [0] ; for item in list { if item > largest { largest = item ; } } largest }
};
}
