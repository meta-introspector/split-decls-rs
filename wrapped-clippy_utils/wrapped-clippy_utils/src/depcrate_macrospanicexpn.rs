// Generated macro for PanicExpn (enum)
macro_rules! Depcrate_macrosPanicExpn {
() => {
// Module: crate::macros
// Provides: {"PanicExpn"}
// Dependencies: {}
# [derive (Debug)] pub enum PanicExpn < 'a > { # [doc = " No arguments - `panic!()`"] Empty , # [doc = " A string literal or any `&str` - `panic!(\"message\")` or `panic!(message)`"] Str (& 'a Expr < 'a >) , # [doc = " A single argument that implements `Display` - `panic!(\"{}\", object)`"] Display (& 'a Expr < 'a >) , # [doc = " Anything else - `panic!(\"error {}: {}\", a, b)`"] Format (& 'a Expr < 'a >) , }
};
}
