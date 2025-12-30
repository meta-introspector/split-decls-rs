// Generated macro for macro_11509 (macro)
macro_rules! Depcrate_writemacro_11509 {
() => {
// Module: crate::write
// Provides: {"macro_11509"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint warns when you use `write!()` with a format"] # [doc = " string that"] # [doc = " ends in a newline."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " You should use `writeln!()` instead, which appends the"] # [doc = " newline."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::fmt::Write;"] # [doc = " # let mut buf = String::new();"] # [doc = " # let name = \"World\";"] # [doc = " write!(buf, \"Hello {}!\\n\", name);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::fmt::Write;"] # [doc = " # let mut buf = String::new();"] # [doc = " # let name = \"World\";"] # [doc = " writeln!(buf, \"Hello {}!\", name);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub WRITE_WITH_NEWLINE , style , "using `write!()` with a format string that ends in a single newline" }
};
}
