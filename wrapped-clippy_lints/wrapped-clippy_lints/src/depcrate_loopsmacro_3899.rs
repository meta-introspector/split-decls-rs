// Generated macro for macro_3899 (macro)
macro_rules! Depcrate_loopsmacro_3899 {
() => {
// Module: crate::loops
// Provides: {"macro_3899"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for while loops comparing floating point values."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " If you increment floating point values, errors can compound,"] # [doc = " so, use integers instead if possible."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " The lint will catch all while loops comparing floating point"] # [doc = " values without regarding the increment."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut x = 0.0;"] # [doc = " while x < 42.0 {"] # [doc = "     x += 1.0;"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut x = 0;"] # [doc = " while x < 42 {"] # [doc = "     x += 1;"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.80.0"] pub WHILE_FLOAT , nursery , "while loops comparing floating point values" }
};
}
