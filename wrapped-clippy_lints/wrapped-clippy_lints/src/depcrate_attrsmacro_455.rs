// Generated macro for macro_455 (macro)
macro_rules! Depcrate_attrsmacro_455 {
() => {
// Module: crate::attrs
// Provides: {"macro_455"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of the `#[allow]` attribute and suggests replacing it with"] # [doc = " the `#[expect]` attribute (See [RFC 2383](https://rust-lang.github.io/rfcs/2383-lint-reasons.html))"] # [doc = ""] # [doc = " This lint only warns outer attributes (`#[allow]`), as inner attributes"] # [doc = " (`#![allow]`) are usually used to enable or disable lints on a global scale."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `#[expect]` attributes suppress the lint emission, but emit a warning, if"] # [doc = " the expectation is unfulfilled. This can be useful to be notified when the"] # [doc = " lint is no longer triggered."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " #[allow(unused_mut)]"] # [doc = " fn foo() -> usize {"] # [doc = "     let mut a = Vec::new();"] # [doc = "     a.len()"] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " #[expect(unused_mut)]"] # [doc = " fn foo() -> usize {"] # [doc = "     let mut a = Vec::new();"] # [doc = "     a.len()"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.70.0"] pub ALLOW_ATTRIBUTES , restriction , "`#[allow]` will not trigger if a warning isn't found. `#[expect]` triggers if there are no warnings." }
};
}
