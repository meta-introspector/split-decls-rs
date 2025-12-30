// Generated macro for macro_4989 (macro)
macro_rules! Depcrate_matchesmacro_4989 {
() => {
// Module: crate::matches
// Provides: {"macro_4989"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of `Err(x)?`."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " The `?` operator is designed to allow calls that"] # [doc = " can fail to be easily chained. For example, `foo()?.bar()` or"] # [doc = " `foo(bar()?)`. Because `Err(x)?` can't be used that way (it will"] # [doc = " always return), it is more clear to write `return Err(x)`."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " fn foo(fail: bool) -> Result<i32, String> {"] # [doc = "     if fail {"] # [doc = "       Err(\"failed\")?;"] # [doc = "     }"] # [doc = "     Ok(0)"] # [doc = " }"] # [doc = " ```"] # [doc = " Could be written:"] # [doc = ""] # [doc = " ```no_run"] # [doc = " fn foo(fail: bool) -> Result<i32, String> {"] # [doc = "     if fail {"] # [doc = "       return Err(\"failed\".into());"] # [doc = "     }"] # [doc = "     Ok(0)"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.38.0"] pub TRY_ERR , restriction , "return errors explicitly rather than hiding them behind a `?`" }
};
}
