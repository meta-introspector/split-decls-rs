// Generated macro for macro_7391 (macro)
macro_rules! Depcrate_misc_earlymacro_7391 {
() => {
// Module: crate::misc_early
// Provides: {"macro_7391"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Warns if an integral constant literal starts with `0`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " In some languages (including the infamous C language"] # [doc = " and most of its"] # [doc = " family), this marks an octal constant. In Rust however, this is a decimal"] # [doc = " constant. This could"] # [doc = " be confusing for both the writer and a reader of the constant."] # [doc = ""] # [doc = " ### Example"] # [doc = ""] # [doc = " In Rust:"] # [doc = " ```no_run"] # [doc = " fn main() {"] # [doc = "     let a = 0123;"] # [doc = "     println!(\"{}\", a);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " prints `123`, while in C:"] # [doc = ""] # [doc = " ```c"] # [doc = " #include <stdio.h>"] # [doc = ""] # [doc = " int main() {"] # [doc = "     int a = 0123;"] # [doc = "     printf(\"%d\\n\", a);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " prints `83` (as `83 == 0o123` while `123 == 0o173`)."] # [clippy :: version = "pre 1.29.0"] pub ZERO_PREFIXED_LITERAL , complexity , "integer literals starting with `0`" }
};
}
