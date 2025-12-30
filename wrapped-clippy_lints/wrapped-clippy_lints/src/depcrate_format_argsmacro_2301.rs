// Generated macro for macro_2301 (macro)
macro_rules! Depcrate_format_argsmacro_2301 {
() => {
// Module: crate::format_args
// Provides: {"macro_2301"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects [formatting parameters] that have no effect on the output of"] # [doc = " `format!()`, `println!()` or similar macros."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Shorter format specifiers are easier to read, it may also indicate that"] # [doc = " an expected formatting operation such as adding padding isn't happening."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " println!(\"{:.}\", 1.0);"] # [doc = ""] # [doc = " println!(\"not padded: {:5}\", format_args!(\"...\"));"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " println!(\"{}\", 1.0);"] # [doc = ""] # [doc = " println!(\"not padded: {}\", format_args!(\"...\"));"] # [doc = " // OR"] # [doc = " println!(\"padded: {:5}\", format!(\"...\"));"] # [doc = " ```"] # [doc = ""] # [doc = " [formatting parameters]: https://doc.rust-lang.org/std/fmt/index.html#formatting-parameters"] # [clippy :: version = "1.66.0"] pub UNUSED_FORMAT_SPECS , complexity , "use of a format specifier that has no effect" }
};
}
