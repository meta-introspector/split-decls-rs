// Generated macro for macro_6993 (macro)
macro_rules! Depcrate_methodsmacro_6993 {
() => {
// Module: crate::methods
// Provides: {"macro_6993"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.skip(x).next()` on iterators."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `.nth(x)` is cleaner"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let some_vec = vec![0, 1, 2, 3];"] # [doc = " let bad_vec = some_vec.iter().skip(3).next();"] # [doc = " let bad_slice = &some_vec[..].iter().skip(3).next();"] # [doc = " ```"] # [doc = " The correct use would be:"] # [doc = " ```no_run"] # [doc = " let some_vec = vec![0, 1, 2, 3];"] # [doc = " let bad_vec = some_vec.iter().nth(3);"] # [doc = " let bad_slice = &some_vec[..].iter().nth(3);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ITER_SKIP_NEXT , style , "using `.skip(x).next()` on an iterator" }
};
}
