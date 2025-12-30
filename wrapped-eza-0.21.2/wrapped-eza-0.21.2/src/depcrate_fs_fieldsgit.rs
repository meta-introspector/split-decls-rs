// Generated macro for Git (struct)
macro_rules! Depcrate_fs_fieldsGit {
() => {
// Module: crate::fs::fields
// Provides: {"Git"}
// Dependencies: {}
# [doc = " A file’s complete Git status. It’s possible to make changes to a file, add"] # [doc = " it to the staging area, then make *more* changes, so we need to list each"] # [doc = " file’s status for both of these."] # [derive (Copy , Clone)] pub struct Git { pub staged : GitStatus , pub unstaged : GitStatus , }
};
}
