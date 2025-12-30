// Generated macro for macro_9016 (macro)
macro_rules! Depcrate_redundant_async_blockmacro_9016 {
() => {
// Module: crate::redundant_async_block
// Provides: {"macro_9016"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `async` block that only returns `await` on a future."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is simpler and more efficient to use the future directly."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let f = async {"] # [doc = "     1 + 2"] # [doc = " };"] # [doc = " let fut = async {"] # [doc = "     f.await"] # [doc = " };"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let f = async {"] # [doc = "     1 + 2"] # [doc = " };"] # [doc = " let fut = f;"] # [doc = " ```"] # [clippy :: version = "1.70.0"] pub REDUNDANT_ASYNC_BLOCK , complexity , "`async { future.await }` can be replaced by `future`" }
};
}
