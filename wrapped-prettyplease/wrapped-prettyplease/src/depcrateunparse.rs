// Generated macro for unparse (function)
macro_rules! Depcrateunparse {
() => {
// Module: crate
// Provides: {"unparse"}
// Dependencies: {}
pub fn unparse (file : & File) -> String { let mut p = Printer :: new () ; p . file (file) ; p . eof () }
};
}
