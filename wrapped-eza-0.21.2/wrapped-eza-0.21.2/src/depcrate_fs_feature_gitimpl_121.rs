// Generated macro for impl_121 (impl)
macro_rules! Depcrate_fs_feature_gitimpl_121 {
() => {
// Module: crate::fs::feature::git
// Provides: {"impl_121"}
// Dependencies: {}
impl f :: SubdirGitRepo { pub fn from_path (dir : & Path , status : bool) -> Self { let path = & reorient (dir) ; if let Ok (repo) = git2 :: Repository :: open (path) { let branch = current_branch (& repo) ; if ! status { return Self { status : None , branch , } ; } match repo . statuses (None) { Ok (es) => { if es . iter () . any (| s | s . status () != git2 :: Status :: IGNORED) { return Self { status : Some (f :: SubdirGitRepoStatus :: GitDirty) , branch , } ; } return Self { status : Some (f :: SubdirGitRepoStatus :: GitClean) , branch , } ; } Err (e) => { error ! ("Error looking up Git statuses: {e:?}") ; } } } f :: SubdirGitRepo { status : if status { Some (f :: SubdirGitRepoStatus :: NoRepo) } else { None } , branch : None , } } }
};
}
