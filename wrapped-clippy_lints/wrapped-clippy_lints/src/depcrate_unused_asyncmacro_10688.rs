// Generated macro for macro_10688 (macro)
macro_rules! Depcrate_unused_asyncmacro_10688 {
() => {
// Module: crate::unused_async
// Provides: {"macro_10688"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for functions that are declared `async` but have no `.await`s inside of them."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Async functions with no async code create overhead, both mentally and computationally."] # [doc = " Callers of async methods either need to be calling from an async function themselves or run it on an executor, both of which"] # [doc = " causes runtime overhead and hassle for the caller."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " async fn get_random_number() -> i64 {"] # [doc = "     4 // Chosen by fair dice roll. Guaranteed to be random."] # [doc = " }"] # [doc = " let number_future = get_random_number();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn get_random_number_improved() -> i64 {"] # [doc = "     4 // Chosen by fair dice roll. Guaranteed to be random."] # [doc = " }"] # [doc = " let number_future = async { get_random_number_improved() };"] # [doc = " ```"] # [clippy :: version = "1.54.0"] pub UNUSED_ASYNC , pedantic , "finds async functions with no await statements" }
};
}
