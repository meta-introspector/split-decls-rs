// Generated macro for macro_3233 (macro)
macro_rules! Depcrate_large_futuresmacro_3233 {
() => {
// Module: crate::large_futures
// Provides: {"macro_3233"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " It checks for the size of a `Future` created by `async fn` or `async {}`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Due to the current [unideal implementation](https://github.com/rust-lang/rust/issues/69826) of `Coroutine`,"] # [doc = " large size of a `Future` may cause stack overflows."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " async fn large_future(_x: [u8; 16 * 1024]) {}"] # [doc = ""] # [doc = " pub async fn trigger() {"] # [doc = "     large_future([0u8; 16 * 1024]).await;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " `Box::pin` the big future instead."] # [doc = ""] # [doc = " ```no_run"] # [doc = " async fn large_future(_x: [u8; 16 * 1024]) {}"] # [doc = ""] # [doc = " pub async fn trigger() {"] # [doc = "     Box::pin(large_future([0u8; 16 * 1024])).await;"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.70.0"] pub LARGE_FUTURES , pedantic , "large future may lead to unexpected stack overflows" }
};
}
