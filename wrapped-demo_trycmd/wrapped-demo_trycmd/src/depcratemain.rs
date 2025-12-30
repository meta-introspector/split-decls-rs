// Generated macro for main (function)
macro_rules! Depcratemain {
() => {
// Module: crate
// Provides: {"main"}
// Dependencies: {}
fn main () { let args : Vec < String > = env :: args () . collect () ; if args . len () == 2 { if env :: var_os ("GOODBYE") . is_some () { println ! ("Goodbye {}!" , args [1]) ; } else { println ! ("Hello {}!" , args [1]) ; } } else { eprintln ! ("Must supply exactly one argument.") ; std :: process :: exit (1) ; } }
};
}
