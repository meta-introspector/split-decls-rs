// Generated macro for macro_7197 (macro)
macro_rules! Depcrate_methodsmacro_7197 {
() => {
// Module: crate::methods
// Provides: {"macro_7197"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.repeat(1)` and suggest the following method for each types."] # [doc = " - `.to_string()` for `str`"] # [doc = " - `.clone()` for `String`"] # [doc = " - `.to_vec()` for `slice`"] # [doc = ""] # [doc = " The lint will evaluate constant expressions and values as arguments of `.repeat(..)` and emit a message if"] # [doc = " they are equivalent to `1`. (Related discussion in [rust-clippy#7306](https://github.com/rust-lang/rust-clippy/issues/7306))"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " For example, `String.repeat(1)` is equivalent to `.clone()`. If cloning"] # [doc = " the string is the intention behind this, `clone()` should be used."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn main() {"] # [doc = "     let x = String::from(\"hello world\").repeat(1);"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn main() {"] # [doc = "     let x = String::from(\"hello world\").clone();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.47.0"] pub REPEAT_ONCE , complexity , "using `.repeat(1)` instead of `String.clone()`, `str.to_string()` or `slice.to_vec()` " }
};
}
