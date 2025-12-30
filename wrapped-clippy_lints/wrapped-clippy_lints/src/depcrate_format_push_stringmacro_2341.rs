// Generated macro for macro_2341 (macro)
macro_rules! Depcrate_format_push_stringmacro_2341 {
() => {
// Module: crate::format_push_string
// Provides: {"macro_2341"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects cases where the result of a `format!` call is"] # [doc = " appended to an existing `String`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Introduces an extra, avoidable heap allocation."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " `format!` returns a `String` but `write!` returns a `Result`."] # [doc = " Thus you are forced to ignore the `Err` variant to achieve the same API."] # [doc = ""] # [doc = " While using `write!` in the suggested way should never fail, this isn't necessarily clear to the programmer."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut s = String::new();"] # [doc = " s += &format!(\"0x{:X}\", 1024);"] # [doc = " s.push_str(&format!(\"0x{:X}\", 1024));"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::fmt::Write as _; // import without risk of name clashing"] # [doc = ""] # [doc = " let mut s = String::new();"] # [doc = " let _ = write!(s, \"0x{:X}\", 1024);"] # [doc = " ```"] # [clippy :: version = "1.62.0"] pub FORMAT_PUSH_STRING , pedantic , "`format!(..)` appended to existing `String`" }
};
}
