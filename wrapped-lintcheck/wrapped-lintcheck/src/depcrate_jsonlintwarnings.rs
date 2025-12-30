// Generated macro for LintWarnings (struct)
macro_rules! Depcrate_jsonLintWarnings {
() => {
// Module: crate::json
// Provides: {"LintWarnings"}
// Dependencies: {}
# [doc = " Container for grouped lint warnings organized by status (added/removed/changed)."] # [derive (Debug)] struct LintWarnings { name : String , added : Vec < LintJson > , removed : Vec < LintJson > , changed : Vec < (LintJson , LintJson) > , }
};
}
