// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let f : Box < dyn Fn () + Send + 'static > = Box :: new (| | println ! ("hi")) ; fn takes_long_type (f : Box < dyn Fn () + Send + 'static >) { } fn returns_long_type () -> Box < dyn Fn () + Send + 'static > { Box :: new (| | ()) } }
};
}
