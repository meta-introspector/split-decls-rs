// Generated macro for macro_280 (macro)
macro_rules! Depcrate_async_yields_asyncmacro_280 {
() => {
// Module: crate::async_yields_async
// Provides: {"macro_280"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for async blocks that yield values of types"] # [doc = " that can themselves be awaited."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " An await is likely missing."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " async fn foo() {}"] # [doc = ""] # [doc = " fn bar() {"] # [doc = "   let x = async {"] # [doc = "     foo()"] # [doc = "   };"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " async fn foo() {}"] # [doc = ""] # [doc = " fn bar() {"] # [doc = "   let x = async {"] # [doc = "     foo().await"] # [doc = "   };"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.48.0"] pub ASYNC_YIELDS_ASYNC , correctness , "async blocks that return a type that can be awaited" }
};
}
