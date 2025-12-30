// Generated macro for macro_6957 (macro)
macro_rules! Depcrate_methodsmacro_6957 {
() => {
// Module: crate::methods
// Provides: {"macro_6957"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `cloned()` on an `Iterator` or `Option` where"] # [doc = " `copied()` could be used instead."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `copied()` is better because it guarantees that the type being cloned"] # [doc = " implements `Copy`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " [1, 2, 3].iter().cloned();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " [1, 2, 3].iter().copied();"] # [doc = " ```"] # [clippy :: version = "1.53.0"] pub CLONED_INSTEAD_OF_COPIED , pedantic , "used `cloned` where `copied` could be used instead" }
};
}
