// Generated macro for macro_8545 (macro)
macro_rules! Depcrate_operatorsmacro_8545 {
() => {
// Module: crate::operators
// Provides: {"macro_8545"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for usage of bitwise and/or operators between booleans, where performance may be improved by using"] # [doc = " a lazy and."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " The bitwise operators do not support short-circuiting, so it may hinder code performance."] # [doc = " Additionally, boolean logic \"masked\" as bitwise logic is not caught by lints like `unnecessary_fold`"] # [doc = ""] # [doc = " ### Known problems"] # [doc = " This lint evaluates only when the right side is determined to have no side effects. At this time, that"] # [doc = " determination is quite conservative."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let (x,y) = (true, false);"] # [doc = " if x & !y {} // where both x and y are booleans"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let (x,y) = (true, false);"] # [doc = " if x && !y {}"] # [doc = " ```"] # [clippy :: version = "1.54.0"] pub NEEDLESS_BITWISE_BOOL , pedantic , "Boolean expressions that use bitwise rather than lazy operators" }
};
}
