// Generated macro for macro_2616 (macro)
macro_rules! Depcrate_functionsmacro_2616 {
() => {
// Module: crate::functions
// Provides: {"macro_2616"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Lints when the name of function parameters from trait impl is"] # [doc = " different than its default implementation."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Using the default name for parameters of a trait method is more consistent."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust"] # [doc = " struct A(u32);"] # [doc = ""] # [doc = " impl PartialEq for A {"] # [doc = "     fn eq(&self, b: &Self) -> bool {"] # [doc = "         self.0 == b.0"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust"] # [doc = " struct A(u32);"] # [doc = ""] # [doc = " impl PartialEq for A {"] # [doc = "     fn eq(&self, other: &Self) -> bool {"] # [doc = "         self.0 == other.0"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.80.0"] pub RENAMED_FUNCTION_PARAMS , restriction , "renamed function parameters in trait implementation" }
};
}
