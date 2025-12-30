// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { if let Err (e) = delegate_cargo () { eprintln ! ("Error: {e}") ; print_usage () ; process :: exit (1) ; } }
};
}
