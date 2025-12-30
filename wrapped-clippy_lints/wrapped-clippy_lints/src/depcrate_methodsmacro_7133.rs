// Generated macro for macro_7133 (macro)
macro_rules! Depcrate_methodsmacro_7133 {
() => {
// Module: crate::methods
// Provides: {"macro_7133"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.clone()` on a ref-counted pointer,"] # [doc = " (`Rc`, `Arc`, `rc::Weak`, or `sync::Weak`), and suggests calling Clone via unified"] # [doc = " function syntax instead (e.g., `Rc::clone(foo)`)."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " Calling `.clone()` on an `Rc`, `Arc`, or `Weak`"] # [doc = " can obscure the fact that only the pointer is being cloned, not the underlying"] # [doc = " data."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::rc::Rc;"] # [doc = " let x = Rc::new(1);"] # [doc = ""] # [doc = " x.clone();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::rc::Rc;"] # [doc = " # let x = Rc::new(1);"] # [doc = " Rc::clone(&x);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub CLONE_ON_REF_PTR , restriction , "using `clone` on a ref-counted pointer" }
};
}
