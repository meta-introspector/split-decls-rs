// Generated macro for macro_11063 (macro)
macro_rules! Depcrate_writemacro_11063 {
() => {
// Module: crate::write
// Provides: {"macro_11063"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for printing on *stderr*. The purpose of this lint"] # [doc = " is to catch debugging remnants."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " People often print on *stderr* while debugging an"] # [doc = " application and might forget to remove those prints afterward."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Only catches `eprint!` and `eprintln!` calls."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " eprintln!(\"Hello world!\");"] # [doc = " ```"] # [clippy :: version = "1.50.0"] pub PRINT_STDERR , restriction , "printing on stderr" }
};
}
