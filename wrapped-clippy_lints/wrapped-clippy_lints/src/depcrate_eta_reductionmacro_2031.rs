// Generated macro for macro_2031 (macro)
macro_rules! Depcrate_eta_reductionmacro_2031 {
() => {
// Module: crate::eta_reduction
// Provides: {"macro_2031"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for closures which just call another function where"] # [doc = " the function can be called directly. `unsafe` functions, calls where types"] # [doc = " get adjusted or where the callee is marked `#[track_caller]` are ignored."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Needlessly creating a closure adds code for no benefit"] # [doc = " and gives the optimizer more work."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " xs.map(|x| foo(x))"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " // where `foo(_)` is a plain function that takes the exact argument type of `x`."] # [doc = " xs.map(foo)"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub REDUNDANT_CLOSURE , style , "redundant closures, i.e., `|a| foo(a)` (which can be written as just `foo`)" }
};
}
