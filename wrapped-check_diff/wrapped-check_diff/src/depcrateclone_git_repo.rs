// Generated macro for clone_git_repo (function)
macro_rules! Depcrateclone_git_repo {
() => {
// Module: crate
// Provides: {"clone_git_repo"}
// Dependencies: {}
# [doc = " Clone a git repository"] # [doc = ""] # [doc = " Parameters:"] # [doc = " url: git clone url"] # [doc = " dest: directory where the repo should be cloned"] pub fn clone_git_repo (url : & str , dest : & Path) -> Result < () , GitError > { let git_cmd = Command :: new ("git") . env ("GIT_TERMINAL_PROMPT" , "0") . args (["clone" , "--quiet" , url , "--depth" , "1" , dest . to_str () . unwrap () ,]) . output () ? ; if ! git_cmd . status . success () { let error = GitError :: FailedClone { stdout : git_cmd . stdout , stderr : git_cmd . stderr , } ; return Err (error) ; } info ! ("Successfully clone repository.") ; return Ok (()) ; }
};
}
