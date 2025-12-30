// Generated macro for macro_11018 (macro)
macro_rules! Depcrate_visibilitymacro_11018 {
() => {
// Module: crate::visibility
// Provides: {"macro_11018"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `pub(<loc>)` with `in`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Consistency. Use it or don't, just be consistent about it."] # [doc = ""] # [doc = " Also see the `pub_without_shorthand` lint for an alternative."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " pub(super) type OptBox<T> = Option<Box<T>>;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " pub(in super) type OptBox<T> = Option<Box<T>>;"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub PUB_WITH_SHORTHAND , restriction , "disallows usage of `pub(<loc>)`, without `in`" }
};
}
