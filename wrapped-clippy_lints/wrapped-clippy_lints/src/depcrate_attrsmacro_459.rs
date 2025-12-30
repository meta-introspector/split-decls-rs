// Generated macro for macro_459 (macro)
macro_rules! Depcrate_attrsmacro_459 {
() => {
// Module: crate::attrs
// Provides: {"macro_459"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `#[cfg_attr(feature = \"cargo-clippy\", ...)]` and for"] # [doc = " `#[cfg(feature = \"cargo-clippy\")]` and suggests to replace it with"] # [doc = " `#[cfg_attr(clippy, ...)]` or `#[cfg(clippy)]`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This feature has been deprecated for years and shouldn't be used anymore."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #[cfg(feature = \"cargo-clippy\")]"] # [doc = " struct Bar;"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #[cfg(clippy)]"] # [doc = " struct Bar;"] # [doc = " ```"] # [clippy :: version = "1.78.0"] pub DEPRECATED_CLIPPY_CFG_ATTR , suspicious , "usage of `cfg(feature = \"cargo-clippy\")` instead of `cfg(clippy)`" }
};
}
