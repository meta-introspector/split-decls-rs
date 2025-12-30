// Generated macro for macro_1215 (macro)
macro_rules! Depcrate_collapsible_ifmacro_1215 {
() => {
// Module: crate::collapsible_if
// Provides: {"macro_1215"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for nested `if` statements which can be collapsed"] # [doc = " by `&&`-combining their conditions."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Each `if`-statement adds one level of nesting, which"] # [doc = " makes code look more complex than it really is."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let (x, y) = (true, true);"] # [doc = " if x {"] # [doc = "     if y {"] # [doc = "         // …"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let (x, y) = (true, true);"] # [doc = " if x && y {"] # [doc = "     // …"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub COLLAPSIBLE_IF , style , "nested `if`s that can be collapsed (e.g., `if x { if y { ... } }`" }
};
}
