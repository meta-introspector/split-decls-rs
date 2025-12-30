// Generated macro for macro_11510 (macro)
macro_rules! Depcrate_writemacro_11510 {
() => {
// Module: crate::write
// Provides: {"macro_11510"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint warns about the use of literals as `write!`/`writeln!` args."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using literals as `writeln!` args is inefficient"] # [doc = " (c.f., https://github.com/matthiaskrgr/rust-str-bench) and unnecessary"] # [doc = " (i.e., just put the literal in the format string)"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::fmt::Write;"] # [doc = " # let mut buf = String::new();"] # [doc = " writeln!(buf, \"{}\", \"foo\");"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::fmt::Write;"] # [doc = " # let mut buf = String::new();"] # [doc = " writeln!(buf, \"foo\");"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub WRITE_LITERAL , style , "writing a literal with a format string" }
};
}
