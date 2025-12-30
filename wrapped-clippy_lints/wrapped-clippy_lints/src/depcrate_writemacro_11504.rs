// Generated macro for macro_11504 (macro)
macro_rules! Depcrate_writemacro_11504 {
() => {
// Module: crate::write
// Provides: {"macro_11504"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for printing on *stdout*. The purpose of this lint"] # [doc = " is to catch debugging remnants."] # [doc = ""] # [doc = " ### Why restrict this?"] # [doc = " People often print on *stdout* while debugging an"] # [doc = " application and might forget to remove those prints afterward."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Only catches `print!` and `println!` calls."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " println!(\"Hello world!\");"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub PRINT_STDOUT , restriction , "printing on stdout" }
};
}
