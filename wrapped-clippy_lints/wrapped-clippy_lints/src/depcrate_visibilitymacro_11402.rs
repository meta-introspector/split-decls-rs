// Generated macro for macro_11402 (macro)
macro_rules! Depcrate_visibilitymacro_11402 {
() => {
// Module: crate::visibility
// Provides: {"macro_11402"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `pub(<loc>)` without `in`."] # [doc = ""] # [doc = " Note: As you cannot write a module's path in `pub(<loc>)`, this will only trigger on"] # [doc = " `pub(super)` and the like."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Consistency. Use it or don't, just be consistent about it."] # [doc = ""] # [doc = " Also see the `pub_with_shorthand` lint for an alternative."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " pub(in super) type OptBox<T> = Option<Box<T>>;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " pub(super) type OptBox<T> = Option<Box<T>>;"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub PUB_WITHOUT_SHORTHAND , restriction , "disallows usage of `pub(in <loc>)` with `in`" }
};
}
