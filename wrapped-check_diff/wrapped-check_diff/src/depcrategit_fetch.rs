// Generated macro for git_fetch (function)
macro_rules! Depcrategit_fetch {
() => {
// Module: crate
// Provides: {"git_fetch"}
// Dependencies: {}
pub fn git_fetch (branch_name : & str) -> Result < () , GitError > { let git_cmd = Command :: new ("git") . args (["fetch" , "feature" , branch_name]) . output () ? ; if ! git_cmd . status . success () { let error = GitError :: FailedFetch { stdout : git_cmd . stdout , stderr : git_cmd . stderr , } ; return Err (error) ; } info ! ("Successfully fetched: {branch_name}") ; return Ok (()) ; }
};
}
