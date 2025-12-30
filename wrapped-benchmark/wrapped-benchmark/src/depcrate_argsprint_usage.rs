// Generated macro for print_usage (function)
macro_rules! Depcrate_argsprint_usage {
() => {
// Module: crate::args
// Provides: {"print_usage"}
// Dependencies: {}
fn print_usage (names : & [& str] , error_msg : Option < String >) -> ! { if let Some (error) = error_msg { println ! ("{error}") ; } println ! ("Usage: {} {}" , env :: args () . next () . unwrap () , names . join (" ")) ; println ! ("Each argument can be a single value or a range in the form start:end or \
         start:end:step") ; process :: exit (1) ; }
};
}
