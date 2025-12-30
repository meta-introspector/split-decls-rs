// Generated macro for macro_1445 (macro)
macro_rules! Depcrate_dereferencemacro_1445 {
() => {
// Module: crate::dereference
// Provides: {"macro_1445"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for address of operations (`&`) that are going to"] # [doc = " be dereferenced immediately by the compiler."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Suggests that the receiver of the expression borrows"] # [doc = " the expression."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The lint cannot tell when the implementation of a trait"] # [doc = " for `&T` and `T` do different things. Removing a borrow"] # [doc = " in such a case can change the semantics of the code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn fun(_a: &i32) {}"] # [doc = ""] # [doc = " let x: &i32 = &&&&&&5;"] # [doc = " fun(&x);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # fn fun(_a: &i32) {}"] # [doc = " let x: &i32 = &5;"] # [doc = " fun(x);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NEEDLESS_BORROW , style , "taking a reference that is going to be automatically dereferenced" }
};
}
