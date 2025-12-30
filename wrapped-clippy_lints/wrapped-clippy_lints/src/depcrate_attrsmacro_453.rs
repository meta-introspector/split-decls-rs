// Generated macro for macro_453 (macro)
macro_rules! Depcrate_attrsmacro_453 {
() => {
// Module: crate::attrs
// Provides: {"macro_453"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `#[cfg_attr(rustfmt, rustfmt_skip)]` and suggests to replace it"] # [doc = " with `#[rustfmt::skip]`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Since tool_attributes ([rust-lang/rust#44690](https://github.com/rust-lang/rust/issues/44690))"] # [doc = " are stable now, they should be used instead of the old `cfg_attr(rustfmt)` attributes."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " This lint doesn't detect crate level inner attributes, because they get"] # [doc = " processed before the PreExpansionPass lints get executed. See"] # [doc = " [#3123](https://github.com/rust-lang/rust-clippy/pull/3123#issuecomment-422321765)"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[cfg_attr(rustfmt, rustfmt_skip)]"] # [doc = " fn main() { }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[rustfmt::skip]"] # [doc = " fn main() { }"] # [doc = " ```"] # [clippy :: version = "1.32.0"] pub DEPRECATED_CFG_ATTR , complexity , "usage of `cfg_attr(rustfmt)` instead of tool attributes" }
};
}
