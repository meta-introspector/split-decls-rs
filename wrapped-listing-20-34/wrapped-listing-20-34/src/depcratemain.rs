// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let handlers = vec ! [returns_closure () , returns_initialized_closure (123)] ; for handler in handlers { let output = handler (5) ; println ! ("{output}") ; } }
};
}
