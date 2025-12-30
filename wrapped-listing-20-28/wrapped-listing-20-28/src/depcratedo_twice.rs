// Generated macro for do_twice (function)
macro_rules! Depcratedo_twice {
() => {
// Module: crate
// Provides: {"do_twice"}
// Dependencies: {}
fn do_twice (f : fn (i32) -> i32 , arg : i32) -> i32 { f (arg) + f (arg) }
};
}
