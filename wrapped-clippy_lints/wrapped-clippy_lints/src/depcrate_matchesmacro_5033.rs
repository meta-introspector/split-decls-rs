// Generated macro for macro_5033 (macro)
macro_rules! Depcrate_matchesmacro_5033 {
() => {
// Module: crate::matches
// Provides: {"macro_5033"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for unnecessary `match` or match-like `if let` returns for `Option` and `Result`"] # [doc = " when function signatures are the same."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This `match` block does nothing and might not be what the coder intended."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,ignore"] # [doc = " fn foo() -> Result<(), i32> {"] # [doc = "     match result {"] # [doc = "         Ok(val) => Ok(val),"] # [doc = "         Err(err) => Err(err),"] # [doc = "     }"] # [doc = " }"] # [doc = ""] # [doc = " fn bar() -> Option<i32> {"] # [doc = "     if let Some(val) = option {"] # [doc = "         Some(val)"] # [doc = "     } else {"] # [doc = "         None"] # [doc = "     }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Could be replaced as"] # [doc = ""] # [doc = " ```rust,ignore"] # [doc = " fn foo() -> Result<(), i32> {"] # [doc = "     result"] # [doc = " }"] # [doc = ""] # [doc = " fn bar() -> Option<i32> {"] # [doc = "     option"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.61.0"] pub NEEDLESS_MATCH , complexity , "`match` or match-like `if let` that are unnecessary" }
};
}
