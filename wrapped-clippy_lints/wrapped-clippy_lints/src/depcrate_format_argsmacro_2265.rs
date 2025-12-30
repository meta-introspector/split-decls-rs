// Generated macro for macro_2265 (macro)
macro_rules! Depcrate_format_argsmacro_2265 {
() => {
// Module: crate::format_args
// Provides: {"macro_2265"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects `format!` within the arguments of another macro that does"] # [doc = " formatting such as `format!` itself, `write!` or `println!`. Suggests"] # [doc = " inlining the `format!` call."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The recommended code is both shorter and avoids a temporary allocation."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::panic::Location;"] # [doc = " println!(\"error: {}\", format!(\"something failed at {}\", Location::caller()));"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::panic::Location;"] # [doc = " println!(\"error: something failed at {}\", Location::caller());"] # [doc = " ```"] # [clippy :: version = "1.58.0"] pub FORMAT_IN_FORMAT_ARGS , perf , "`format!` used in a macro that does formatting" }
};
}
