// Generated macro for print_env (function)
macro_rules! Depcrateprint_env {
() => {
// Module: crate
// Provides: {"print_env"}
// Dependencies: {}
pub fn print_env () { eprintln ! () ; eprintln ! ("Arguments:") ; for argument in env :: args () { eprintln ! ("{argument}") ; } eprintln ! () ; eprintln ! ("Environment variables:") ; for (key , value) in env :: vars () { eprintln ! ("{key}: {value}") ; } }
};
}
