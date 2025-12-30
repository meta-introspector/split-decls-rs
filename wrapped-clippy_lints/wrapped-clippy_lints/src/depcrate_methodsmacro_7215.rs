// Generated macro for macro_7215 (macro)
macro_rules! Depcrate_methodsmacro_7215 {
() => {
// Module: crate::methods
// Provides: {"macro_7215"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.map(|_| format!(..)).collect::<String>()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This allocates a new string for every element in the iterator."] # [doc = " This can be done more efficiently by creating the `String` once and appending to it in `Iterator::fold`,"] # [doc = " using either the `write!` macro which supports exactly the same syntax as the `format!` macro,"] # [doc = " or concatenating with `+` in case the iterator yields `&str`/`String`."] # [doc = ""] # [doc = " Note also that `write!`-ing into a `String` can never fail, despite the return type of `write!` being `std::fmt::Result`,"] # [doc = " so it can be safely ignored or unwrapped."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn hex_encode(bytes: &[u8]) -> String {"] # [doc = "     bytes.iter().map(|b| format!(\"{b:02X}\")).collect()"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " use std::fmt::Write;"] # [doc = " fn hex_encode(bytes: &[u8]) -> String {"] # [doc = "     bytes.iter().fold(String::new(), |mut output, b| {"] # [doc = "         let _ = write!(output, \"{b:02X}\");"] # [doc = "         output"] # [doc = "     })"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub FORMAT_COLLECT , pedantic , "`format!`ing every element in a collection, then collecting the strings into a new `String`" }
};
}
