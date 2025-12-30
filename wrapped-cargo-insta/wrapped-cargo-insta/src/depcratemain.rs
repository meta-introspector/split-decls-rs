// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { if let Err (err) = cli :: run () { let exit_code = if let Some (exit) = err . downcast_ref :: < utils :: QuietExit > () { exit . 0 } else { println ! ("{} {}" , style ("error:") . red () . bold () , err) ; 1 } ; std :: process :: exit (exit_code) ; } }
};
}
