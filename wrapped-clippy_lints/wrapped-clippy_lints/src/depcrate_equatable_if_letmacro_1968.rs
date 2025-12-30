// Generated macro for macro_1968 (macro)
macro_rules! Depcrate_equatable_if_letmacro_1968 {
() => {
// Module: crate::equatable_if_let
// Provides: {"macro_1968"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for pattern matchings that can be expressed using equality."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " * It reads better and has less cognitive load because equality won't cause binding."] # [doc = " * It is a [Yoda condition](https://en.wikipedia.org/wiki/Yoda_conditions). Yoda conditions are widely"] # [doc = " criticized for increasing the cognitive load of reading the code."] # [doc = " * Equality is a simple bool expression and can be merged with `&&` and `||` and"] # [doc = " reuse if blocks"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " if let Some(2) = x {"] # [doc = "     do_thing();"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " if x == Some(2) {"] # [doc = "     do_thing();"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.57.0"] pub EQUATABLE_IF_LET , nursery , "using pattern matching instead of equality" }
};
}
