// Generated macro for macro_2080 (macro)
macro_rules! Depcrate_excessive_boolsmacro_2080 {
() => {
// Module: crate::excessive_bools
// Provides: {"macro_2080"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for excessive use of"] # [doc = " bools in function definitions."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Calls to such functions"] # [doc = " are confusing and error prone, because it's"] # [doc = " hard to remember argument order and you have"] # [doc = " no type system support to back you up. Using"] # [doc = " two-variant enums instead of bools often makes"] # [doc = " API easier to use."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " fn f(is_round: bool, is_hot: bool) { ... }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " enum Shape {"] # [doc = "     Round,"] # [doc = "     Spiky,"] # [doc = " }"] # [doc = ""] # [doc = " enum Temperature {"] # [doc = "     Hot,"] # [doc = "     IceCold,"] # [doc = " }"] # [doc = ""] # [doc = " fn f(shape: Shape, temperature: Temperature) { ... }"] # [doc = " ```"] # [clippy :: version = "1.43.0"] pub FN_PARAMS_EXCESSIVE_BOOLS , pedantic , "using too many bools in function parameters" }
};
}
