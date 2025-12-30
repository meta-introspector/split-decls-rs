// Generated macro for git_remote_add (function)
macro_rules! Depcrategit_remote_add {
() => {
// Module: crate
// Provides: {"git_remote_add"}
// Dependencies: {}
pub fn git_remote_add (url : & str) -> Result < () , GitError > { let git_cmd = Command :: new ("git") . args (["remote" , "add" , "feature" , url]) . output () ? ; if ! git_cmd . status . success () { let error = GitError :: FailedRemoteAdd { stdout : git_cmd . stdout , stderr : git_cmd . stderr , } ; return Err (error) ; } info ! ("Successfully added remote: {url}") ; return Ok (()) ; }
};
}
