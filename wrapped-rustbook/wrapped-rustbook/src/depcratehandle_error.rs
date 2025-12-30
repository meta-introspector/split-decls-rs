// Generated macro for handle_error (function)
macro_rules! Depcratehandle_error {
() => {
// Module: crate
// Provides: {"handle_error"}
// Dependencies: {}
fn handle_error (error : mdbook :: errors :: Error) -> ! { eprintln ! ("Error: {}" , error) ; for cause in error . chain () . skip (1) { eprintln ! ("\tCaused By: {}" , cause) ; } std :: process :: exit (101) ; }
};
}
