// Generated macro for quit (function)
macro_rules! Depcratequit {
() => {
// Module: crate
// Provides: {"quit"}
// Dependencies: {}
fn quit (why : & str) -> ! { println_err ! ("{}" , why) ; process :: exit (0) }
};
}
