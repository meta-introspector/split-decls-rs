// Generated macro for LintLevelSets (struct)
macro_rules! Depcrate_levelsLintLevelSets {
() => {
// Module: crate::levels
// Provides: {"LintLevelSets"}
// Dependencies: {}
# [doc = " Collection of lint levels for the whole crate."] # [doc = " This is used by AST-based lints, which do not"] # [doc = " wait until we have built HIR to be emitted."] # [derive (Debug)] struct LintLevelSets { # [doc = " Linked list of specifications."] list : IndexVec < LintStackIndex , LintSet > , }
};
}
