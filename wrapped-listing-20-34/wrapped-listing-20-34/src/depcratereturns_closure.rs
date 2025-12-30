// Generated macro for returns_closure (function)
macro_rules! Depcratereturns_closure {
() => {
// Module: crate
// Provides: {"returns_closure"}
// Dependencies: {}
fn returns_closure () -> Box < dyn Fn (i32) -> i32 > { Box :: new (| x | x + 1) }
};
}
