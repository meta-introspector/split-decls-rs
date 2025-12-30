// Generated macro for macro_7019 (macro)
macro_rules! Depcrate_methodsmacro_7019 {
() => {
// Module: crate::methods
// Provides: {"macro_7019"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for instances of `map(f)` where `f` is the identity function."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It can be written more concisely without the call to `map`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = [1, 2, 3];"] # [doc = " let y: Vec<_> = x.iter().map(|x| x).map(|x| 2*x).collect();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = [1, 2, 3];"] # [doc = " let y: Vec<_> = x.iter().map(|x| 2*x).collect();"] # [doc = " ```"] # [clippy :: version = "1.47.0"] pub MAP_IDENTITY , complexity , "using iterator.map(|x| x)" }
};
}
