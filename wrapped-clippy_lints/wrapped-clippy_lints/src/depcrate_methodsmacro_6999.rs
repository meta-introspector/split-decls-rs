// Generated macro for macro_6999 (macro)
macro_rules! Depcrate_methodsmacro_6999 {
() => {
// Module: crate::methods
// Provides: {"macro_6999"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for the use of `.cloned().collect()` on slice to"] # [doc = " create a `Vec`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `.to_vec()` is clearer"] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let s = [1, 2, 3, 4, 5];"] # [doc = " let s2: Vec<isize> = s[..].iter().cloned().collect();"] # [doc = " ```"] # [doc = " The better use would be:"] # [doc = " ```no_run"] # [doc = " let s = [1, 2, 3, 4, 5];"] # [doc = " let s2: Vec<isize> = s.to_vec();"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub ITER_CLONED_COLLECT , style , "using `.cloned().collect()` on slice to create a `Vec`" }
};
}
