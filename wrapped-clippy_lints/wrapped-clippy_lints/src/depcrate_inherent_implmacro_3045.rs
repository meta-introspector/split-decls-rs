// Generated macro for macro_3045 (macro)
macro_rules! Depcrate_inherent_implmacro_3045 {
() => {
// Module: crate::inherent_impl
// Provides: {"macro_3045"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for multiple inherent implementations of a struct"] # [doc = ""] # [doc = " The config option controls the scope in which multiple inherent `impl` blocks for the same"] # [doc = " struct are linted, allowing values of `module` (only within the same module), `file`"] # [doc = " (within the same file), or `crate` (anywhere in the crate, default)."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Splitting the implementation of a type makes the code harder to navigate."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct X;"] # [doc = " impl X {"] # [doc = "     fn one() {}"] # [doc = " }"] # [doc = " impl X {"] # [doc = "     fn other() {}"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Could be written:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " struct X;"] # [doc = " impl X {"] # [doc = "     fn one() {}"] # [doc = "     fn other() {}"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MULTIPLE_INHERENT_IMPL , restriction , "Multiple inherent impl that could be grouped" }
};
}
