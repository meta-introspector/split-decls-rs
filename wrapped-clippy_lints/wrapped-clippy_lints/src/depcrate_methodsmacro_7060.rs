// Generated macro for macro_7060 (macro)
macro_rules! Depcrate_methodsmacro_7060 {
() => {
// Module: crate::methods
// Provides: {"macro_7060"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = ""] # [doc = " Checks for `Command::arg()` invocations that look like they"] # [doc = " should be multiple arguments instead, such as `arg(\"-t ext2\")`."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = ""] # [doc = " `Command::arg()` does not split arguments by space. An argument like `arg(\"-t ext2\")`"] # [doc = " will be passed as a single argument to the command,"] # [doc = " which is likely not what was intended."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " std::process::Command::new(\"echo\").arg(\"-n hello\").spawn().unwrap();"] # [doc = " ```"] # [doc = " Use instead:"] # [doc = " ```no_run"] # [doc = " std::process::Command::new(\"echo\").args([\"-n\", \"hello\"]).spawn().unwrap();"] # [doc = " ```"] # [clippy :: version = "1.69.0"] pub SUSPICIOUS_COMMAND_ARG_SPACE , suspicious , "single command line argument that looks like it should be multiple arguments" }
};
}
