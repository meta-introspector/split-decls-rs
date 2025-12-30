// Generated macro for get_repo_name (function)
macro_rules! Depcrate_utilsget_repo_name {
() => {
// Module: crate::utils
// Provides: {"get_repo_name"}
// Dependencies: {}
fn get_repo_name (url : & str) -> String { let repo_name = url . split ('/') . next_back () . unwrap () ; match repo_name . strip_suffix (".git") { Some (n) => n . to_string () , None => repo_name . to_string () , } }
};
}
