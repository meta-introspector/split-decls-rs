// Generated macro for macro_11066 (macro)
macro_rules! Depcrate_writemacro_11066 {
() => {
// Module: crate::write
// Provides: {"macro_11066"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint warns when you use `writeln!(buf, \"\")` to"] # [doc = " print a newline."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " You should use `writeln!(buf)`, which is simpler."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::fmt::Write;"] # [doc = " # let mut buf = String::new();"] # [doc = " writeln!(buf, \"\");"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::fmt::Write;"] # [doc = " # let mut buf = String::new();"] # [doc = " writeln!(buf);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub WRITELN_EMPTY_STRING , style , "using `writeln!(buf, \"\")` with an empty string" }
};
}
