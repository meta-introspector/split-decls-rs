// Generated macro for macro_7040 (macro)
macro_rules! Depcrate_methodsmacro_7040 {
() => {
// Module: crate::methods
// Provides: {"macro_7040"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `x.get(0)` instead of"] # [doc = " `x.first()` or `x.front()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `x.first()` for `Vec`s and slices or `x.front()`"] # [doc = " for `VecDeque`s is easier to read and has the same result."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = vec![2, 3, 5];"] # [doc = " let first_element = x.get(0);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = vec![2, 3, 5];"] # [doc = " let first_element = x.first();"] # [doc = " ```"] # [clippy :: version = "1.63.0"] pub GET_FIRST , style , "Using `x.get(0)` when `x.first()` or `x.front()` is simpler" }
};
}
