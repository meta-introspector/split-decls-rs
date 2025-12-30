// Generated macro for macro_2267 (macro)
macro_rules! Depcrate_format_argsmacro_2267 {
() => {
// Module: crate::format_args
// Provides: {"macro_2267"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for [`ToString::to_string`](https://doc.rust-lang.org/std/string/trait.ToString.html#tymethod.to_string)"] # [doc = " applied to a type that implements [`Display`](https://doc.rust-lang.org/std/fmt/trait.Display.html)"] # [doc = " in a macro that does formatting."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Since the type implements `Display`, the use of `to_string` is"] # [doc = " unnecessary."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::panic::Location;"] # [doc = " println!(\"error: something failed at {}\", Location::caller().to_string());"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::panic::Location;"] # [doc = " println!(\"error: something failed at {}\", Location::caller());"] # [doc = " ```"] # [clippy :: version = "1.58.0"] pub TO_STRING_IN_FORMAT_ARGS , perf , "`to_string` applied to a type that implements `Display` in format args" }
};
}
