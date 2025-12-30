// Generated macro for macro_2323 (macro)
macro_rules! Depcrate_format_implmacro_2323 {
() => {
// Module: crate::format_impl
// Provides: {"macro_2323"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for format trait implementations (e.g. `Display`) with a recursive call to itself"] # [doc = " which uses `self` as a parameter."] # [doc = " This is typically done indirectly with the `write!` macro or with `to_string()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This will lead to infinite recursion and a stack overflow."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " ```no_run"] # [doc = " use std::fmt;"] # [doc = ""] # [doc = " struct Structure(i32);"] # [doc = " impl fmt::Display for Structure {"] # [doc = "     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {"] # [doc = "         write!(f, \"{}\", self.to_string())"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::fmt;"] # [doc = ""] # [doc = " struct Structure(i32);"] # [doc = " impl fmt::Display for Structure {"] # [doc = "     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {"] # [doc = "         write!(f, \"{}\", self.0)"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.48.0"] pub RECURSIVE_FORMAT_IMPL , correctness , "Format trait method called while implementing the same Format trait" }
};
}
