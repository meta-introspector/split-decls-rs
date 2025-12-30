// Generated macro for macro_11503 (macro)
macro_rules! Depcrate_writemacro_11503 {
() => {
// Module: crate::write
// Provides: {"macro_11503"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " This lint warns when you use `print!()` with a format"] # [doc = " string that ends in a newline."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " You should use `println!()` instead, which appends the"] # [doc = " newline."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " # let name = \"World\";"] # [doc = " print!(\"Hello {}!\\n\", name);"] # [doc = " ```"] # [doc = " use println!() instead"] # [doc = " ```no_run"] # [doc = " # let name = \"World\";"] # [doc = " println!(\"Hello {}!\", name);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub PRINT_WITH_NEWLINE , style , "using `print!()` with a format string that ends in a single newline" }
};
}
