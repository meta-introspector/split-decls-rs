// Generated macro for macro_6994 (macro)
macro_rules! Depcrate_methodsmacro_6994 {
() => {
// Module: crate::methods
// Provides: {"macro_6994"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `.drain(..)` on `Vec` and `VecDeque` for iteration."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " `.into_iter()` is simpler with better performance."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashSet;"] # [doc = " let mut foo = vec![0, 1, 2, 3];"] # [doc = " let bar: HashSet<usize> = foo.drain(..).collect();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # use std::collections::HashSet;"] # [doc = " let foo = vec![0, 1, 2, 3];"] # [doc = " let bar: HashSet<usize> = foo.into_iter().collect();"] # [doc = " ```"] # [clippy :: version = "1.61.0"] pub ITER_WITH_DRAIN , nursery , "replace `.drain(..)` with `.into_iter()`" }
};
}
