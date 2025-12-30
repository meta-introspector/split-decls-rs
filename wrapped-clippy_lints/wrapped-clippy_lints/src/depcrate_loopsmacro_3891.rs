// Generated macro for macro_3891 (macro)
macro_rules! Depcrate_loopsmacro_3891 {
() => {
// Module: crate::loops
// Provides: {"macro_3891"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects `loop + match` combinations that are easier"] # [doc = " written as a `while let` loop."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The `while let` loop is usually shorter and more"] # [doc = " readable."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```rust,no_run"] # [doc = " let y = Some(1);"] # [doc = " loop {"] # [doc = "     let x = match y {"] # [doc = "         Some(x) => x,"] # [doc = "         None => break,"] # [doc = "     };"] # [doc = "     // .."] # [doc = " }"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```rust,no_run"] # [doc = " let y = Some(1);"] # [doc = " while let Some(x) = y {"] # [doc = "     // .."] # [doc = " };"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub WHILE_LET_LOOP , complexity , "`loop { if let { ... } else break }`, which can be written as a `while let` loop" }
};
}
