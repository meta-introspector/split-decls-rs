// Generated macro for macro_7152 (macro)
macro_rules! Depcrate_methodsmacro_7152 {
() => {
// Module: crate::methods
// Provides: {"macro_7152"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `into_iter` calls on references which should be replaced by `iter`"] # [doc = " or `iter_mut`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Readability. Calling `into_iter` on a reference will not move out its"] # [doc = " content into the resulting iterator, which is confusing. It is better just call `iter` or"] # [doc = " `iter_mut` directly."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let vec = vec![3, 4, 5];"] # [doc = " (&vec).into_iter();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let vec = vec![3, 4, 5];"] # [doc = " (&vec).iter();"] # [doc = " ```"] # [clippy :: version = "1.32.0"] pub INTO_ITER_ON_REF , style , "using `.into_iter()` on a reference" }
};
}
