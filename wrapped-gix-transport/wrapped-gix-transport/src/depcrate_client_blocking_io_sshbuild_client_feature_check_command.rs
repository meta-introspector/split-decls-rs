// Generated macro for build_client_feature_check_command (function)
macro_rules! Depcrate_client_blocking_io_sshbuild_client_feature_check_command {
() => {
// Module: crate::client::blocking_io::ssh
// Provides: {"build_client_feature_check_command"}
// Dependencies: {}
# [allow (clippy :: result_large_err)] fn build_client_feature_check_command (ssh_cmd : & OsStr , url : & Url , disallow_shell : bool) -> Result < Command , Error > { let mut prepare = gix_command :: prepare (ssh_cmd) . stderr (Stdio :: null ()) . stdout (Stdio :: null ()) . stdin (Stdio :: null ()) . command_may_be_shell_script () . arg ("-G") . arg (match url . host_as_argument () { Usable (host) => host , Dangerous (host) => Err (Error :: AmbiguousHostName { host : host . into () }) ? , Absent => panic ! ("BUG: host should always be present in SSH URLs") , }) ; if disallow_shell { prepare . use_shell = false ; } Ok (prepare . into ()) }
};
}
