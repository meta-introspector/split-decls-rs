// Generated macro for exe_info (function)
macro_rules! Depcrate_env_gitexe_info {
() => {
// Module: crate::env::git
// Provides: {"exe_info"}
// Dependencies: {}
fn exe_info () -> Option < BString > { let mut cmd = git_cmd (EXE_NAME . into ()) ; gix_trace :: debug ! (cmd = ? cmd , "invoking git for installation config path") ; let cmd_output = match cmd . output () { Ok (out) => out . stdout , # [cfg (windows)] Err (err) if err . kind () == std :: io :: ErrorKind :: NotFound => { let executable = ALTERNATIVE_LOCATIONS . iter () . find_map (| prefix | { let candidate = prefix . join (EXE_NAME) ; candidate . is_file () . then_some (candidate) }) ? ; gix_trace :: debug ! (cmd = ? cmd , "invoking git for installation config path in alternate location") ; git_cmd (executable) . output () . ok () ? . stdout } Err (_) => return None , } ; first_file_from_config_with_origin (cmd_output . as_slice () . into ()) . map (ToOwned :: to_owned) }
};
}
