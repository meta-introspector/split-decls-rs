// Generated macro for macro_632 (macro)
macro_rules! Depcrate_box_defaultmacro_632 {
() => {
// Module: crate::box_default
// Provides: {"macro_632"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " checks for `Box::new(Default::default())`, which can be written as"] # [doc = " `Box::default()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `Box::default()` is equivalent and more concise."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x: Box<String> = Box::new(Default::default());"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x: Box<String> = Box::default();"] # [doc = " ```"] # [clippy :: version = "1.66.0"] pub BOX_DEFAULT , style , "Using Box::new(T::default()) instead of Box::default()" }
};
}
