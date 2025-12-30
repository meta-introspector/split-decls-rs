// Generated macro for print_usage (function)
macro_rules! Depcrateprint_usage {
() => {
// Module: crate
// Provides: {"print_usage"}
// Dependencies: {}
fn print_usage () { let name = env :: args () . next () . unwrap () ; eprintln ! ("Usage:") ; eprintln ! ("  {} --version <toolchain-name>" , name) ; eprintln ! ("  {} +<toolchain-name> [...]" , name) ; eprintln ! ("  {} +all [...]" , name) ; }
};
}
