// Generated macro for macro_2749 (macro)
macro_rules! Depcrate_ifsmacro_2749 {
() => {
// Module: crate::ifs
// Provides: {"macro_2749"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for consecutive `if`s with the same condition."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This is probably a copy & paste error."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```ignore"] # [doc = " if a == b {"] # [doc = "     …"] # [doc = " } else if a == b {"] # [doc = "     …"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Note that this lint ignores all conditions with a function call as it could"] # [doc = " have side effects:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " if foo() {"] # [doc = "     …"] # [doc = " } else if foo() { // not linted"] # [doc = "     …"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub IFS_SAME_COND , correctness , "consecutive `if`s with the same condition" }
};
}
