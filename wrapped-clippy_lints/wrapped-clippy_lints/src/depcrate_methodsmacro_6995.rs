// Generated macro for macro_6995 (macro)
macro_rules! Depcrate_methodsmacro_6995 {
() => {
// Module: crate::methods
// Provides: {"macro_6995"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `x.get(x.len() - 1)` instead of"] # [doc = " `x.last()`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Using `x.last()` is easier to read and has the same"] # [doc = " result."] # [doc = ""] # [doc = " Note that using `x[x.len() - 1]` is semantically different from"] # [doc = " `x.last()`.  Indexing into the array will panic on out-of-bounds"] # [doc = " accesses, while `x.get()` and `x.last()` will return `None`."] # [doc = ""] # [doc = " There is another lint (get_unwrap) that covers the case of using"] # [doc = " `x.get(index).unwrap()` instead of `x[index]`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let x = vec![2, 3, 5];"] # [doc = " let last_element = x.get(x.len() - 1);"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let x = vec![2, 3, 5];"] # [doc = " let last_element = x.last();"] # [doc = " ```"] # [clippy :: version = "1.37.0"] pub GET_LAST_WITH_LEN , complexity , "Using `x.get(x.len() - 1)` when `x.last()` is correct and simpler" }
};
}
