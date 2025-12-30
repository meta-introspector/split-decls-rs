// Generated macro for macro_3956 (macro)
macro_rules! Depcrate_loopsmacro_3956 {
() => {
// Module: crate::loops
// Provides: {"macro_3956"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks whether a for loop has a single element."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " There is no reason to have a loop of a"] # [doc = " single element."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let item1 = 2;"] # [doc = " for item in &[item1] {"] # [doc = "     println!(\"{}\", item);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let item1 = 2;"] # [doc = " let item = &item1;"] # [doc = " println!(\"{}\", item);"] # [doc = " ```"] # [clippy :: version = "1.49.0"] pub SINGLE_ELEMENT_LOOP , complexity , "there is no reason to have a single element loop" }
};
}
