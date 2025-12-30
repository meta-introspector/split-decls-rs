// Generated macro for returns_initialized_closure (function)
macro_rules! Depcratereturns_initialized_closure {
() => {
// Module: crate
// Provides: {"returns_initialized_closure"}
// Dependencies: {}
fn returns_initialized_closure (init : i32) -> impl Fn (i32) -> i32 { move | x | x + init }
};
}
