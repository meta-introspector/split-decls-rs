// Generated macro for macro_11065 (macro)
macro_rules! Depcrate_writemacro_11065 {
() => {
// Module: crate::write
// Provides: {"macro_11065"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint warns about the use of literals as `print!`/`println!` args."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using literals as `println!` args is inefficient"] # [doc = " (c.f., https://github.com/matthiaskrgr/rust-str-bench) and unnecessary"] # [doc = " (i.e., just put the literal in the format string)"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " println!(\"{}\", \"foo\");"] # [doc = " ```"] # [doc = " use the literal without formatting:"] # [doc = " ```no_run"] # [doc = " println!(\"foo\");"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub PRINT_LITERAL , style , "printing a literal with a format string" }
};
}
