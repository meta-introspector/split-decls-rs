// Generated macro for ApplyLocation (enum)
macro_rules! Depcrate_applyApplyLocation {
() => {
// Module: crate::apply
// Provides: {"ApplyLocation"}
// Dependencies: {}
# [doc = " Possible application locations for git_apply"] # [doc = " see <https://libgit2.org/libgit2/#HEAD/type/git_apply_options>"] # [derive (Copy , Clone , Debug)] pub enum ApplyLocation { # [doc = " Apply the patch to the workdir"] WorkDir , # [doc = " Apply the patch to the index"] Index , # [doc = " Apply the patch to both the working directory and the index"] Both , }
};
}
