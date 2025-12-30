// Generated macro for check_valgrind (function)
macro_rules! Depcratecheck_valgrind {
() => {
// Module: crate
// Provides: {"check_valgrind"}
// Dependencies: {}
fn check_valgrind () -> bool { let result = Command :: new ("valgrind") . arg ("--tool=cachegrind") . arg ("--version") . stdout (Stdio :: null ()) . stderr (Stdio :: null ()) . status () ; match result { Err (e) => { println ! ("Unexpected error while launching valgrind. Error: {}" , e) ; false } Ok (status) => { if status . success () { true } else { println ! ("Failed to launch valgrind. Error: {}. Please ensure that valgrind is installed and on the $PATH." , status) ; false } } } }
};
}
