// Generated macro for macro_2123 (macro)
macro_rules! Depcrate_exitmacro_2123 {
() => {
// Module: crate::exit
// Provides: {"macro_2123"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Detects calls to the `exit()` function that are not in the `main` function. Calls to `exit()`"] # [doc = " immediately terminate the program."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " `exit()` immediately terminates the program with no information other than an exit code."] # [doc = " This provides no means to troubleshoot a problem, and may be an unexpected side effect."] # [doc = ""] # [doc = " Codebases may use this lint to require that all exits are performed either by panicking"] # [doc = " (which produces a message, a code location, and optionally a backtrace)"] # [doc = " or by calling `exit()` from `main()` (which is a single place to look)."] # [doc = ""] # [doc = " ### Good example"] # [doc = " ```no_run"] # [doc = " fn main() {"] # [doc = "     std::process::exit(0);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " ### Bad example"] # [doc = " ```no_run"] # [doc = " fn main() {"] # [doc = "     other_function();"] # [doc = " }"] # [doc = ""] # [doc = " fn other_function() {"] # [doc = "     std::process::exit(0);"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " Use instead:"] # [doc = ""] # [doc = " ```ignore"] # [doc = " // To provide a stacktrace and additional information"] # [doc = " panic!(\"message\");"] # [doc = ""] # [doc = " // or a main method with a return"] # [doc = " fn main() -> Result<(), i32> {"] # [doc = "     Ok(())"] # [doc = " }"] # [doc = " ```"] # [clippy :: version = "1.41.0"] pub EXIT , restriction , "detects `std::process::exit` calls outside of `main`" }
};
}
