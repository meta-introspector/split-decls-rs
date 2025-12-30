// Generated macro for git_clone (function)
macro_rules! Depcrate_utilsgit_clone {
() => {
// Module: crate::utils
// Provides: {"git_clone"}
// Dependencies: {}
pub fn git_clone (to_clone : & str , dest : Option < & Path > , shallow_clone : bool ,) -> Result < CloneResult , String > { let repo_name = get_repo_name (to_clone) ; let tmp : PathBuf ; let dest = match dest { Some (dest) => dest , None => { tmp = repo_name . clone () . into () ; & tmp } } ; git_clone_inner (to_clone , dest , shallow_clone , repo_name) }
};
}
