// Generated macro for macro_3385 (macro)
macro_rules! Depcrate_let_underscoremacro_3385 {
() => {
// Module: crate::let_underscore
// Provides: {"macro_3385"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `let _ = <expr>` where the resulting type of expr implements `Future`"] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " Futures must be polled for work to be done. The original intention was most likely to await the future"] # [doc = " and ignore the resulting value."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " async fn foo() -> Result<(), ()> {"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " let _ = foo();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " # async fn context() {"] # [doc = " async fn foo() -> Result<(), ()> {"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " let _ = foo().await;"] # [doc = " # }"] # [doc = " ```"] # [clippy :: version = "1.67.0"] pub LET_UNDERSCORE_FUTURE , suspicious , "non-binding `let` on a future" }
};
}
