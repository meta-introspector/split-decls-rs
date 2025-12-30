// Generated macro for macro_2938 (macro)
macro_rules! Depcrate_inherent_implmacro_2938 {
() => {
// Module: crate::inherent_impl
// Provides: {"macro_2938"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for multiple inherent implementations of a struct"] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Splitting the implementation of a type makes the code harder to navigate."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " struct X;"] # [doc = " impl X {"] # [doc = "     fn one() {}"] # [doc = " }"] # [doc = " impl X {"] # [doc = "     fn other() {}"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Could be written:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " struct X;"] # [doc = " impl X {"] # [doc = "     fn one() {}"] # [doc = "     fn other() {}"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MULTIPLE_INHERENT_IMPL , restriction , "Multiple inherent impl that could be grouped" }
};
}
