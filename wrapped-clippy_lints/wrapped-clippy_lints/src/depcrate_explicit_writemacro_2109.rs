// Generated macro for macro_2109 (macro)
macro_rules! Depcrate_explicit_writemacro_2109 {
() => {
// Module: crate::explicit_write
// Provides: {"macro_2109"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `write!()` / `writeln()!` which can be"] # [doc = " replaced with `(e)print!()` / `(e)println!()`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `(e)println!` is clearer and more concise"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::io::Write;"] # [doc = " # let bar = \"furchtbar\";"] # [doc = " writeln!(&mut std::io::stderr(), \"foo: {:?}\", bar).unwrap();"] # [doc = " writeln!(&mut std::io::stdout(), \"foo: {:?}\", bar).unwrap();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::io::Write;"] # [doc = " # let bar = \"furchtbar\";"] # [doc = " eprintln!(\"foo: {:?}\", bar);"] # [doc = " println!(\"foo: {:?}\", bar);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub EXPLICIT_WRITE , complexity , "using the `write!()` family of functions instead of the `print!()` family of functions, when using the latter would work" }
};
}
