// Generated macro for macro_722 (macro)
macro_rules! Depcrate_cargomacro_722 {
() => {
// Module: crate::cargo
// Provides: {"macro_722"}
// Dependencies: {}
declare_clippy_lint ! { # [doc = " ### What it does"] # [doc = " Checks to see if multiple versions of a crate are being"] # [doc = " used."] # [doc = ""] # [doc = " ### Why is this bad?"] # [doc = " This bloats the size of targets, and can lead to"] # [doc = " confusing error messages when structs or traits are used interchangeably"] # [doc = " between different versions of a crate."] # [doc = ""] # [doc = " ### Known problems"] # [doc = " Because this can be caused purely by the dependencies"] # [doc = " themselves, it's not always possible to fix this issue."] # [doc = " In those cases, you can allow that specific crate using"] # [doc = " the `allowed_duplicate_crates` configuration option."] # [doc = ""] # [doc = " ### Example"] # [doc = " ```toml"] # [doc = " # This will pull in both winapi v0.3.x and v0.2.x, triggering a warning."] # [doc = " [dependencies]"] # [doc = " ctrlc = \"=3.1.0\""] # [doc = " ansi_term = \"=0.11.0\""] # [doc = " ```"] # [clippy :: version = "pre 1.29.0"] pub MULTIPLE_CRATE_VERSIONS , cargo , "multiple versions of the same crate being used" }
};
}
