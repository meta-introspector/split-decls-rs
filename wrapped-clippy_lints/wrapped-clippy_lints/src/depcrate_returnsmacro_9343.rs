// Generated macro for macro_9343 (macro)
macro_rules! Depcrate_returnsmacro_9343 {
() => {
// Module: crate::returns
// Provides: {"macro_9343"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for return statements on `Err` paired with the `?` operator."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `return` is unnecessary."] # [doc = ""] # [doc = " Returns may be used to add attributes to the return expression. Return"] # [doc = " statements with attributes are therefore be accepted by this lint."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " fn foo(x: usize) -> Result<(), Box<dyn Error>> {"] # [doc = "     if x == 0 {"] # [doc = "         return Err(...)?;"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc = " simplify to"] # [doc = " ```rust,ignore"] # [doc = " fn foo(x: usize) -> Result<(), Box<dyn Error>> {"] # [doc = "     if x == 0 {"] # [doc = "         Err(...)?;"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [doc = " if paired with `try_err`, use instead:"] # [doc = " ```rust,ignore"] # [doc = " fn foo(x: usize) -> Result<(), Box<dyn Error>> {"] # [doc = "     if x == 0 {"] # [doc = "         return Err(...);"] # [doc = "     }"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.73.0"] pub NEEDLESS_RETURN_WITH_QUESTION_MARK , style , "using a return statement like `return Err(expr)?;` where removing it would suffice" }
};
}
