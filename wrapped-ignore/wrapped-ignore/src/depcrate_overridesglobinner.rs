// Generated macro for GlobInner (enum)
macro_rules! Depcrate_overridesGlobInner {
() => {
// Module: crate::overrides
// Provides: {"GlobInner"}
// Dependencies: {}
# [derive (Clone , Debug)] # [allow (dead_code)] enum GlobInner < 'a > { # [doc = " No glob matched, but the file path should still be ignored."] UnmatchedIgnore , # [doc = " A glob matched."] Matched (& 'a gitignore :: Glob) , }
};
}
