// Generated macro for add_submodule (function)
macro_rules! Depcrate_gitadd_submodule {
() => {
// Module: crate::git
// Provides: {"add_submodule"}
// Dependencies: {}
# [doc = " *(`git2`)* Add a git submodule to the repository"] pub fn add_submodule < 'a > (repo : & 'a git2 :: Repository , url : & str , path : & Path ,) -> git2 :: Submodule < 'a > { let path = path . to_str () . unwrap () . replace (r"\" , "/") ; let mut s = t ! (repo . submodule (url , Path :: new (& path) , false)) ; let subrepo = t ! (s . open ()) ; default_repo_cfg (& subrepo) ; t ! (subrepo . remote_add_fetch ("origin" , "refs/heads/*:refs/heads/*")) ; let mut origin = t ! (subrepo . find_remote ("origin")) ; t ! (origin . fetch (& Vec ::< String >:: new () , None , None)) ; t ! (subrepo . checkout_head (None)) ; t ! (s . add_finalize ()) ; s }
};
}
