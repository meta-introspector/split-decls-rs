// Generated macro for macro_11506 (macro)
macro_rules! Depcrate_writemacro_11506 {
() => {
// Module: crate::write
// Provides: {"macro_11506"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `Debug` formatting. The purpose of this"] # [doc = " lint is to catch debugging remnants."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " The purpose of the `Debug` trait is to facilitate debugging Rust code,"] # [doc = " and [no guarantees are made about its output][stability]."] # [doc = " It should not be used in user-facing output."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let foo = \"bar\";"] # [doc = " println!(\"{:?}\", foo);"] # [doc = " ```"] # [doc = ""] # [doc = " [stability]: https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html#stability"] # [clippy :: version = "pre 1.29.0"] pub USE_DEBUG , restriction , "use of `Debug`-based formatting" }
};
}
