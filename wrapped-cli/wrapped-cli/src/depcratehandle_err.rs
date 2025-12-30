// Generated macro for handle_err (function)
macro_rules! Depcratehandle_err {
() => {
// Module: crate
// Provides: {"handle_err"}
// Dependencies: {}
fn handle_err (error : & Error) { let code = error . get_exit_code () ; let code_formatted = format ! ("[E{code:0>4}]") . red () . bold () ; eprintln ! ("{code_formatted} {error}") ; exit (code) ; }
};
}
