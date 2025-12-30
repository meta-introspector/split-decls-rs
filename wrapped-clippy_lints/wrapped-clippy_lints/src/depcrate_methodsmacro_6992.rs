// Generated macro for macro_6992 (macro)
macro_rules! Depcrate_methodsmacro_6992 {
() => {
// Module: crate::methods
// Provides: {"macro_6992"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.iter().nth()`/`.iter_mut().nth()` on standard library types that have"] # [doc = " equivalent `.get()`/`.get_mut()` methods."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `.get()` and `.get_mut()` are equivalent but more concise."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let some_vec = vec![0, 1, 2, 3];"] # [doc = " let bad_vec = some_vec.iter().nth(3);"] # [doc = " let bad_slice = &some_vec[..].iter().nth(3);"] # [doc = " ```"] # [doc = " The correct use would be:"] # [doc = " ```no_run"] # [doc = " let some_vec = vec![0, 1, 2, 3];"] # [doc = " let bad_vec = some_vec.get(3);"] # [doc = " let bad_slice = &some_vec[..].get(3);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ITER_NTH , style , "using `.iter().nth()` on a standard library type with O(1) element access" }
};
}
