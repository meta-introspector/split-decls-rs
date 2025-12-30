// Generated macro for macro_2060 (macro)
macro_rules! Depcrate_eta_reductionmacro_2060 {
() => {
// Module: crate::eta_reduction
// Provides: {"macro_2060"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for closures which only invoke a method on the closure"] # [doc = " argument and can be replaced by referencing the method directly."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's unnecessary to create the closure."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " Some('a').map(|s| s.to_uppercase());"] # [doc = " ```"] # [doc = " may be rewritten as"] # [doc = " ```rust,ignore"] # [doc = " Some('a').map(char::to_uppercase);"] # [doc = " ```"] # [clippy :: version = "1.35.0"] pub REDUNDANT_CLOSURE_FOR_METHOD_CALLS , pedantic , "redundant closures for method calls" }
};
}
