// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { if let Err (error) = try_main () { let _ = writeln ! (io :: stderr () , "{}" , error) ; process :: exit (1) ; } }
};
}
