// Generated macro for macro_4402 (macro)
macro_rules! Depcrate_manual_retainmacro_4402 {
() => {
// Module: crate::manual_retain
// Provides: {"macro_4402"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for code to be replaced by `.retain()`."] # [doc = " ### Why is this bad?"] # [doc = " `.retain()` is simpler and avoids needless allocation."] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " let mut vec = vec![0, 1, 2];"] # [doc = " vec = vec.iter().filter(|&x| x % 2 == 0).copied().collect();"] # [doc = " vec = vec.into_iter().filter(|x| x % 2 == 0).collect();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " let mut vec = vec![0, 1, 2];"] # [doc = " vec.retain(|x| x % 2 == 0);"] # [doc = " vec.retain(|x| x % 2 == 0);"] # [doc = " ```"] # [clippy :: version = "1.64.0"] pub MANUAL_RETAIN , perf , "`retain()` is simpler and the same functionalities" }
};
}
