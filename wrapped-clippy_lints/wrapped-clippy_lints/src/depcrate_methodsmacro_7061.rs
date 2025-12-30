// Generated macro for macro_7061 (macro)
macro_rules! Depcrate_methodsmacro_7061 {
() => {
// Module: crate::methods
// Provides: {"macro_7061"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.drain(..)` for the sole purpose of clearing a container."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This creates an unnecessary iterator that is dropped immediately."] # [doc = ""] # [doc = " Calling `.clear()` also makes the intent clearer."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut v = vec![1, 2, 3];"] # [doc = " v.drain(..);"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut v = vec![1, 2, 3];"] # [doc = " v.clear();"] # [doc = " ```"] # [clippy :: version = "1.70.0"] pub CLEAR_WITH_DRAIN , nursery , "calling `drain` in order to `clear` a container" }
};
}
