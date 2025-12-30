// Generated macro for macro_7193 (macro)
macro_rules! Depcrate_methodsmacro_7193 {
() => {
// Module: crate::methods
// Provides: {"macro_7193"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks for duplicate open options as well as combinations"] # [doc = " that make no sense."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " In the best case, the code will be harder to read than"] # [doc = " necessary. I don't know the worst case."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```no_run"] # [doc = " use std::fs::OpenOptions;"] # [doc = ""] # [doc = " OpenOptions::new().read(true).truncate(true);"] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub NONSENSICAL_OPEN_OPTIONS , correctness , "nonsensical combination of options for opening a file" }
};
}
