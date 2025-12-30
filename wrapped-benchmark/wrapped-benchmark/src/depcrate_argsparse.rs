// Generated macro for parse (function)
macro_rules! Depcrate_argsparse {
() => {
// Module: crate::args
// Provides: {"parse"}
// Dependencies: {}
pub fn parse (names : & [& str]) -> Vec < ArgRange > { let args = env :: args () . skip (1) . collect :: < Vec < _ > > () ; if args . is_empty () { print_usage (names , None) ; } if args . len () != names . len () { print_usage (names , Some (format ! ("Invalid number of arguments (expected {}, got {})" , names . len () , args . len ())) ,) ; } let mut result = vec ! [] ; for (name , value) in names . iter () . zip (args) { result . push (parse_one (names , name , & value)) ; } result }
};
}
