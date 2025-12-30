// Generated macro for macro_2270 (macro)
macro_rules! Depcrate_format_argsmacro_2270 {
() => {
// Module: crate::format_args
// Provides: {"macro_2270"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects [pointer format] as well as `Debug` formatting of raw pointers or function pointers"] # [doc = " or any types that have a derived `Debug` impl that recursively contains them."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " The addresses are only useful in very specific contexts, and certain projects may want to keep addresses of"] # [doc = " certain data structures or functions from prying hacker eyes as an additional line of security."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The lint currently only looks through derived `Debug` implementations. Checking whether a manual"] # [doc = " implementation prints an address is left as an exercise to the next lint implementer."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let foo = &0_u32;"] # [doc = " fn bar() {}"] # [doc = " println!(\"{:p}\", foo);"] # [doc = " let _ = format!(\"{:?}\", &(bar as fn()));"] # [doc = " ```"] # [doc = ""] # [doc = " [pointer format]: https://doc.rust-lang.org/std/fmt/index.html#formatting-traits"] # [clippy :: version = "1.89.0"] pub POINTER_FORMAT , restriction , "formatting a pointer" }
};
}
