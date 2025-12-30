// Generated macro for returns_closure (function)
macro_rules! Depcratereturns_closure {
() => {
// Module: crate
// Provides: {"returns_closure"}
// Dependencies: {}
fn returns_closure (init : i32) -> impl Fn (i32) -> i32 { move | x | x + init }
};
}
