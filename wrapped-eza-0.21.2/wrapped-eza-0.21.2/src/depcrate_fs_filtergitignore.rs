// Generated macro for GitIgnore (enum)
macro_rules! Depcrate_fs_filterGitIgnore {
() => {
// Module: crate::fs::filter
// Provides: {"GitIgnore"}
// Dependencies: {}
# [doc = " Whether to ignore or display files that Git would ignore."] # [derive (PartialEq , Eq , Debug , Copy , Clone)] pub enum GitIgnore { # [doc = " Ignore files that Git would ignore."] CheckAndIgnore , # [doc = " Display files, even if Git would ignore them."] Off , }
};
}
