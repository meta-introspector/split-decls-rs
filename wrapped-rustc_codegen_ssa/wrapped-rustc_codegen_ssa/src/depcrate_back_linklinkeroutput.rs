// Generated macro for LinkerOutput (struct)
macro_rules! Depcrate_back_linkLinkerOutput {
() => {
// Module: crate::back::link
// Provides: {"LinkerOutput"}
// Dependencies: {}
# [derive (LintDiagnostic)] # [diag (codegen_ssa_linker_output)] # [doc = " Translating this is kind of useless. We don't pass translation flags to the linker, so we'd just"] # [doc = " end up with inconsistent languages within the same diagnostic."] struct LinkerOutput { inner : String , }
};
}
