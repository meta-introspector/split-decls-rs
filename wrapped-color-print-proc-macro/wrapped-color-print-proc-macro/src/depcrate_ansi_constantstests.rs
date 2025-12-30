// Generated macro for tests (module)
macro_rules! Depcrate_ansi_constantstests {
() => {
// Module: crate::ansi_constants
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use super :: * ; # [test] fn ansi_code () { assert_eq ! (generate_ansi_code (& [0]) , "\u{1b}[0m") ; assert_eq ! (generate_ansi_code (& [31]) , "\u{1b}[31m") ; assert_eq ! (generate_ansi_code (& [38 , 5 , 1]) , "\u{1b}[38;5;1m") ; } }
};
}
