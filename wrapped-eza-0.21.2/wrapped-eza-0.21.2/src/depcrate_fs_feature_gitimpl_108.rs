// Generated macro for impl_108 (impl)
macro_rules! Depcrate_fs_feature_gitimpl_108 {
() => {
// Module: crate::fs::feature::git
// Provides: {"impl_108"}
// Dependencies: {}
impl FromIterator < PathBuf > for GitCache { fn from_iter < I > (iter : I) -> Self where I : IntoIterator < Item = PathBuf > , { let iter = iter . into_iter () ; let mut git = Self { repos : Vec :: with_capacity (iter . size_hint () . 0) , misses : Vec :: new () , } ; if let Ok (path) = env :: var ("GIT_DIR") { let flags = git2 :: RepositoryOpenFlags :: NO_SEARCH | git2 :: RepositoryOpenFlags :: NO_DOTGIT ; match GitRepo :: discover (path . into () , flags) { Ok (repo) => { debug ! ("Opened GIT_DIR repo") ; git . repos . push (repo) ; } Err (miss) => { git . misses . push (miss) ; } } } for path in iter { if git . misses . contains (& path) { debug ! ("Skipping {:?} because it already came back Gitless" , path) ; } else if git . repos . iter () . any (| e | e . has_path (& path)) { debug ! ("Skipping {:?} because we already queried it" , path) ; } else { let flags = git2 :: RepositoryOpenFlags :: FROM_ENV ; match GitRepo :: discover (path , flags) { Ok (r) => { if let Some (r2) = git . repos . iter_mut () . find (| e | e . has_workdir (& r . workdir)) { debug ! ("Adding to existing repo (workdir matches with {:?})" , r2 . workdir) ; r2 . extra_paths . push (r . original_path) ; continue ; } debug ! ("Discovered new Git repo") ; git . repos . push (r) ; } Err (miss) => { git . misses . push (miss) ; } } } } git } }
};
}
