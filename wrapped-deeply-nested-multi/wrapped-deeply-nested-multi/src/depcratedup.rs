// Generated macro for dup (function)
macro_rules! Depcratedup {
() => {
// Module: crate
// Provides: {"dup"}
// Dependencies: {}
fn dup (f : impl Fn (i32) -> i32) -> impl Fn (i32) -> i32 { move | a | f (a * 2) }
};
}
