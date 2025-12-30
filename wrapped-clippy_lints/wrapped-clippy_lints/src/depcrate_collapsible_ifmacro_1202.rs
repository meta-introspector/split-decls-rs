// Generated macro for macro_1202 (macro)
macro_rules! Depcrate_collapsible_ifmacro_1202 {
() => {
// Module: crate::collapsible_if
// Provides: {"macro_1202"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for collapsible `else { if ... }` expressions"] # [doc = " that can be collapsed to `else if ...`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Each `if`-statement adds one level of nesting, which"] # [doc = " makes code look more complex than it really is."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = ""] # [doc = " if x {"] # [doc = "     …"] # [doc = " } else {"] # [doc = "     if y {"] # [doc = "         …"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Should be written:"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " if x {"] # [doc = "     …"] # [doc = " } else if y {"] # [doc = "     …"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.51.0"] pub COLLAPSIBLE_ELSE_IF , style , "nested `else`-`if` expressions that can be collapsed (e.g., `else { if x { ... } }`)" }
};
}
