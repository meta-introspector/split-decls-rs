// Generated macro for macro_7668 (macro)
macro_rules! Depcrate_needless_borrows_for_generic_argsmacro_7668 {
() => {
// Module: crate::needless_borrows_for_generic_args
// Provides: {"macro_7668"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for borrow operations (`&`) that are used as a generic argument to a"] # [doc = " function when the borrowed value could be used."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Suggests that the receiver of the expression borrows"] # [doc = " the expression."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The lint cannot tell when the implementation of a trait"] # [doc = " for `&T` and `T` do different things. Removing a borrow"] # [doc = " in such a case can change the semantics of the code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn f(_: impl AsRef<str>) {}"] # [doc = ""] # [doc = " let x = \"foo\";"] # [doc = " f(&x);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn f(_: impl AsRef<str>) {}"] # [doc = ""] # [doc = " let x = \"foo\";"] # [doc = " f(x);"] # [doc = " ```"] # [clippy :: version = "1.74.0"] pub NEEDLESS_BORROWS_FOR_GENERIC_ARGS , style , "taking a reference that is going to be automatically dereferenced" }
};
}
