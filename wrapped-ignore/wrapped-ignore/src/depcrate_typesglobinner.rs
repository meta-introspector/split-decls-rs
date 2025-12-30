// Generated macro for GlobInner (enum)
macro_rules! Depcrate_typesGlobInner {
() => {
// Module: crate::types
// Provides: {"GlobInner"}
// Dependencies: {}
# [derive (Clone , Debug)] enum GlobInner < 'a > { # [doc = " No glob matched, but the file path should still be ignored."] UnmatchedIgnore , # [doc = " A glob matched."] Matched { # [doc = " The file type definition which provided the glob."] def : & 'a FileTypeDef , } , }
};
}
