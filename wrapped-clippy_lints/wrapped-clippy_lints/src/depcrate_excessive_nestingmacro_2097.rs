// Generated macro for macro_2097 (macro)
macro_rules! Depcrate_excessive_nestingmacro_2097 {
() => {
// Module: crate::excessive_nesting
// Provides: {"macro_2097"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for blocks which are nested beyond a certain threshold."] # [doc = ""] # [doc = " Note: Even though this lint is warn-by-default, it will only trigger if a maximum nesting level is defined in the clippy.toml file."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It can severely hinder readability."] # [doc = ""] # [doc = " ### Example"] # [doc = " An example clippy.toml configuration:"] # [doc = " ```toml"] # [doc = " # clippy.toml"] # [doc = " excessive-nesting-threshold = 3"] # [doc = " ```"] # [doc = " ```rust,ignore"] # [doc = " // lib.rs"] # [doc = " pub mod a {"] # [doc = "     pub struct X;"] # [doc = "     impl X {"] # [doc = "         pub fn run(&self) {"] # [doc = "             if true {"] # [doc = "                 // etc..."] # [doc = "             }"] # [doc = "         }"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " // a.rs"] # [doc = " fn private_run(x: &X) {"] # [doc = "     if true {"] # [doc = "         // etc..."] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " pub struct X;"] # [doc = " impl X {"] # [doc = "     pub fn run(&self) {"] # [doc = "         private_run(self);"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " ```rust,ignore"] # [doc = " // lib.rs"] # [doc = " pub mod a;"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub EXCESSIVE_NESTING , complexity , "checks for blocks nested beyond a certain threshold" }
};
}
