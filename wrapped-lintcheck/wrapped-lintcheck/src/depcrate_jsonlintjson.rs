// Generated macro for LintJson (struct)
macro_rules! Depcrate_jsonLintJson {
() => {
// Module: crate::json
// Provides: {"LintJson"}
// Dependencies: {}
# [derive (Debug , Deserialize , Serialize)] struct LintJson { # [doc = " The lint name e.g. `clippy::bytes_nth`"] name : String , # [doc = " The filename and line number e.g. `anyhow-1.0.86/src/error.rs:42`"] file_line : String , file_url : String , rendered : String , }
};
}
