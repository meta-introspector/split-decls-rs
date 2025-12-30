// Generated macro for macro_2298 (macro)
macro_rules! Depcrate_format_argsmacro_2298 {
() => {
// Module: crate::format_args
// Provides: {"macro_2298"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `Debug` formatting (`{:?}`) applied to an `OsStr` or `Path`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Rust doesn't guarantee what `Debug` formatting looks like, and it could"] # [doc = " change in the future. `OsStr`s and `Path`s can be `Display` formatted"] # [doc = " using their `display` methods."] # [doc = ""] # [doc = " Furthermore, with `Debug` formatting, certain characters are escaped."] # [doc = " Thus, a `Debug` formatted `Path` is less likely to be clickable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::path::Path;"] # [doc = " let path = Path::new(\"...\");"] # [doc = " println!(\"The path is {:?}\", path);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::path::Path;"] # [doc = " let path = Path::new(\"…\");"] # [doc = " println!(\"The path is {}\", path.display());"] # [doc = " ```"] # [clippy :: version = "1.87.0"] pub UNNECESSARY_DEBUG_FORMATTING , pedantic , "`Debug` formatting applied to an `OsStr` or `Path` when `.display()` is available" }
};
}
