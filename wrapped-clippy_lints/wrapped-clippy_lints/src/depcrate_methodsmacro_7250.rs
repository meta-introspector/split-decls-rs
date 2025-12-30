// Generated macro for macro_7250 (macro)
macro_rules! Depcrate_methodsmacro_7250 {
() => {
// Module: crate::methods
// Provides: {"macro_7250"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " This lint warns on calling `io::Error::new(..)` with a kind of"] # [doc = " `io::ErrorKind::Other`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Since Rust 1.74, there's the `io::Error::other(_)` shortcut."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::io;"] # [doc = " let _ = io::Error::new(io::ErrorKind::Other, \"bad\".to_string());"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let _ = std::io::Error::other(\"bad\".to_string());"] # [doc = " ```"] # [clippy :: version = "1.87.0"] pub IO_OTHER_ERROR , style , "calling `std::io::Error::new(std::io::ErrorKind::Other, _)`" }
};
}
