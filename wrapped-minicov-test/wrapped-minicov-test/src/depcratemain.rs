// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { do_stuff (false) ; let mut coverage = vec ! [] ; unsafe { minicov :: capture_coverage (& mut coverage) . unwrap () ; } std :: fs :: write ("output.profraw" , coverage) . unwrap () ; }
};
}
