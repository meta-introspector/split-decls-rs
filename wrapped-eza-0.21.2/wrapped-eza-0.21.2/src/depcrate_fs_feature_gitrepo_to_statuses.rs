// Generated macro for repo_to_statuses (function)
macro_rules! Depcrate_fs_feature_gitrepo_to_statuses {
() => {
// Module: crate::fs::feature::git
// Provides: {"repo_to_statuses"}
// Dependencies: {}
# [doc = " Iterates through a repository’s statuses, consuming it and returning the"] # [doc = " mapping of files to their Git status."] # [doc = " We will have already used the working directory at this point, so it gets"] # [doc = " passed in rather than deriving it from the `Repository` again."] fn repo_to_statuses (repo : & git2 :: Repository , workdir : & Path) -> Git { let mut statuses = Vec :: new () ; info ! ("Getting Git statuses for repo with workdir {:?}" , workdir) ; match repo . statuses (None) { Ok (es) => { for e in es . iter () { # [cfg (target_family = "unix")] let path = workdir . join (Path :: new (OsStr :: from_bytes (e . path_bytes ()))) ; # [cfg (not (target_family = "unix"))] let path = workdir . join (Path :: new (e . path () . unwrap ())) ; let elem = (path , e . status ()) ; statuses . push (elem) ; } statuses . push ((workdir . join (".git") , git2 :: Status :: IGNORED)) ; } Err (e) => { error ! ("Error looking up Git statuses: {:?}" , e) ; } } Git { statuses } }
};
}
