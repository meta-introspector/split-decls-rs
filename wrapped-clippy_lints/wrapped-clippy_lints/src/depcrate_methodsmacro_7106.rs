// Generated macro for macro_7106 (macro)
macro_rules! Depcrate_methodsmacro_7106 {
() => {
// Module: crate::methods
// Provides: {"macro_7106"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `_.cloned().<func>()` where call to `.cloned()` can be postponed."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It's often inefficient to clone all elements of an iterator, when eventually, only some"] # [doc = " of them will be consumed."] # [doc = ""] # [doc = " ### Known Problems"] # [doc = " This `lint` removes the side of effect of cloning items in the iterator."] # [doc = " A code that relies on that side-effect could fail."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " # let vec = vec![\"string\".to_string()];"] # [doc = " vec.iter().cloned().take(10);"] # [doc = " vec.iter().cloned().last();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # let vec = vec![\"string\".to_string()];"] # [doc = " vec.iter().take(10).cloned();"] # [doc = " vec.iter().last().cloned();"] # [doc = " ```"] # [clippy :: version = "1.60.0"] pub ITER_OVEREAGER_CLONED , perf , "using `cloned()` early with `Iterator::iter()` can lead to some performance inefficiencies" }
};
}
