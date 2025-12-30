// Generated macro for macro_3951 (macro)
macro_rules! Depcrate_loopsmacro_3951 {
() => {
// Module: crate::loops
// Provides: {"macro_3951"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for loops that will always `break`, `return` or"] # [doc = " `continue` an outer loop."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This loop never loops, all it does is obfuscating the"] # [doc = " code."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " loop {"] # [doc = "     ..;"] # [doc = "     break;"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NEVER_LOOP , correctness , "any loop that will always `break` or `return`" }
};
}
