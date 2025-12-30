// Generated macro for macro_7642 (macro)
macro_rules! Depcrate_needless_borrowed_refmacro_7642 {
() => {
// Module: crate::needless_borrowed_ref
// Provides: {"macro_7642"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for bindings that needlessly destructure a reference and borrow the inner"] # [doc = " value with `&ref`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This pattern has no effect in almost all cases."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut v = Vec::<String>::new();"] # [doc = " v.iter_mut().filter(|&ref a| a.is_empty());"] # [doc = ""] # [doc = " if let &[ref first, ref second] = v.as_slice() {}"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut v = Vec::<String>::new();"] # [doc = " v.iter_mut().filter(|a| a.is_empty());"] # [doc = ""] # [doc = " if let [first, second] = v.as_slice() {}"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NEEDLESS_BORROWED_REFERENCE , complexity , "destructuring a reference and borrowing the inner value" }
};
}
