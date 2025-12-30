// Generated macro for Outcome (struct)
macro_rules! Depcrate_rewritesOutcome {
() => {
// Module: crate::rewrites
// Provides: {"Outcome"}
// Dependencies: {}
# [doc = " Information collected while handling rewrites of files which may be tracked."] # [derive (Default , Clone , Copy , Debug , PartialEq)] pub struct Outcome { # [doc = " The options used to guide the rewrite tracking. Either fully provided by the caller or retrieved from git configuration."] pub options : Rewrites , # [doc = " The amount of similarity checks that have been conducted to find renamed files and potentially copies."] pub num_similarity_checks : usize , # [doc = " Set to the amount of worst-case rename permutations we didn't search as our limit didn't allow it."] pub num_similarity_checks_skipped_for_rename_tracking_due_to_limit : usize , # [doc = " Set to the amount of worst-case copy permutations we didn't search as our limit didn't allow it."] pub num_similarity_checks_skipped_for_copy_tracking_due_to_limit : usize , }
};
}
