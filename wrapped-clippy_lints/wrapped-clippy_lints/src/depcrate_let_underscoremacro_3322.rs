// Generated macro for macro_3322 (macro)
macro_rules! Depcrate_let_underscoremacro_3322 {
() => {
// Module: crate::let_underscore
// Provides: {"macro_3322"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `let _ = <expr>` without a type annotation, and suggests to either provide one,"] # [doc = " or remove the `let` keyword altogether."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " The `let _ = <expr>` expression ignores the value of `<expr>`, but will continue to do so even"] # [doc = " if the type were to change, thus potentially introducing subtle bugs. By supplying a type"] # [doc = " annotation, one will be forced to re-visit the decision to ignore the value in such cases."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The `_ = <expr>` is not properly supported by some tools (e.g. IntelliJ) and may seem odd"] # [doc = " to many developers. This lint also partially overlaps with the other `let_underscore_*`"] # [doc = " lints."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo() -> Result<u32, ()> {"] # [doc = "     Ok(123)"] # [doc = " }"] # [doc = " let _ = foo();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " fn foo() -> Result<u32, ()> {"] # [doc = "     Ok(123)"] # [doc = " }"] # [doc = " // Either provide a type annotation:"] # [doc = " let _: Result<u32, ()> = foo();"] # [doc = " // …or drop the let keyword:"] # [doc = " _ = foo();"] # [doc = " ```"] # [clippy :: version = "1.69.0"] pub LET_UNDERSCORE_UNTYPED , restriction , "non-binding `let` without a type annotation" }
};
}
