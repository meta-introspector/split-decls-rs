// Generated macro for git_switch (function)
macro_rules! Depcrategit_switch {
() => {
// Module: crate
// Provides: {"git_switch"}
// Dependencies: {}
pub fn git_switch (git_ref : & str , should_detach : bool) -> Result < () , GitError > { let detach_arg = if should_detach { "--detach" } else { "" } ; let args = ["switch" , git_ref , detach_arg] ; let output = Command :: new ("git") . args (args . iter () . filter (| arg | ! arg . is_empty ())) . output () ? ; if ! output . status . success () { tracing :: error ! ("Git switch failed: {output:?}") ; let error = GitError :: FailedSwitch { stdout : output . stdout , stderr : output . stderr , } ; return Err (error) ; } info ! ("Successfully switched to {git_ref}") ; return Ok (()) ; }
};
}
