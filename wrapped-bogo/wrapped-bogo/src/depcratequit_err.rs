// Generated macro for quit_err (function)
macro_rules! Depcratequit_err {
() => {
// Module: crate
// Provides: {"quit_err"}
// Dependencies: {}
fn quit_err (why : & str) -> ! { println_err ! ("{}" , why) ; process :: exit (1) }
};
}
