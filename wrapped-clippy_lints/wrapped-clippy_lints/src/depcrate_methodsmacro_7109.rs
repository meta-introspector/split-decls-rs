// Generated macro for macro_7109 (macro)
macro_rules! Depcrate_methodsmacro_7109 {
() => {
// Module: crate::methods
// Provides: {"macro_7109"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for `.unwrap()` related calls on `Result`s and `Option`s that are constructed."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " It is better to write the value directly without the indirection."] # [doc = ""] # [doc = " ### Examples"] # [doc = " ```no_run"] # [doc = " let val1 = Some(1).unwrap();"] # [doc = " let val2 = Ok::<_, ()>(1).unwrap();"] # [doc = " let val3 = Err::<(), _>(1).unwrap_err();"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let val1 = 1;"] # [doc = " let val2 = 1;"] # [doc = " let val3 = 1;"] # [doc = " ```"] # [clippy :: version = "1.72.0"] pub UNNECESSARY_LITERAL_UNWRAP , complexity , "using `unwrap()` related calls on `Result` and `Option` constructors" }
};
}
