// Generated macro for macro_11400 (macro)
macro_rules! Depcrate_visibilitymacro_11400 {
() => {
// Module: crate::visibility
// Provides: {"macro_11400"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `pub(self)` and `pub(in self)`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's unnecessary, omitting the `pub` entirely will give the same results."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " pub(self) type OptBox<T> = Option<Box<T>>;"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,ignore"] # [doc = " type OptBox<T> = Option<Box<T>>;"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub NEEDLESS_PUB_SELF , style , "checks for usage of `pub(self)` and `pub(in self)`." }
};
}
