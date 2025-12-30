// Generated macro for git_process (function)
macro_rules! Depcrategit_process {
() => {
// Module: crate
// Provides: {"git_process"}
// Dependencies: {}
# [doc = " Run `git $arg_line`, see [`ProcessBuilder`]"] pub fn git_process (arg_line : & str) -> ProcessBuilder { let mut p = process ("git") ; p . arg_line (arg_line) ; p }
};
}
