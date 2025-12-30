// Generated macro for macro_454 (macro)
macro_rules! Depcrate_attrsmacro_454 {
() => {
// Module: crate::attrs
// Provides: {"macro_454"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for attributes that allow lints without a reason."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Justifying each `allow` helps readers understand the reasoning,"] # [doc = " and may allow removing `allow` attributes if their purpose is obsolete."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " #![allow(clippy::some_lint)]"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " #![allow(clippy::some_lint, reason = \"False positive rust-lang/rust-clippy#1002020\")]"] # [doc = " ```"] # [clippy :: version = "1.61.0"] pub ALLOW_ATTRIBUTES_WITHOUT_REASON , restriction , "ensures that all `allow` and `expect` attributes have a reason" }
};
}
