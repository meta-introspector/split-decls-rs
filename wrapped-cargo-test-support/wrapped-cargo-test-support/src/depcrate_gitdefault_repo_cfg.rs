// Generated macro for default_repo_cfg (function)
macro_rules! Depcrate_gitdefault_repo_cfg {
() => {
// Module: crate::git
// Provides: {"default_repo_cfg"}
// Dependencies: {}
fn default_repo_cfg (repo : & git2 :: Repository) { let mut cfg = t ! (repo . config ()) ; t ! (cfg . set_str ("user.email" , "foo@bar.com")) ; t ! (cfg . set_str ("user.name" , "Foo Bar")) ; }
};
}
