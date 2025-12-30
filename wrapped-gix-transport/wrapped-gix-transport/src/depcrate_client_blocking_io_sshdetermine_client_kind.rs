// Generated macro for determine_client_kind (function)
macro_rules! Depcrate_client_blocking_io_sshdetermine_client_kind {
() => {
// Module: crate::client::blocking_io::ssh
// Provides: {"determine_client_kind"}
// Dependencies: {}
# [allow (clippy :: result_large_err)] fn determine_client_kind (known_kind : Option < ProgramKind > , ssh_cmd : & OsStr , url : & Url , disallow_shell : bool ,) -> Result < ProgramKind , Error > { let mut kind = known_kind . unwrap_or_else (| | ProgramKind :: from (ssh_cmd)) ; if known_kind . is_none () && kind == ProgramKind :: Simple { let mut cmd = build_client_feature_check_command (ssh_cmd , url , disallow_shell) ? ; gix_features :: trace :: debug ! (cmd = ? cmd , "invoking `ssh` for feature check") ; kind = if cmd . status () . ok () . is_some_and (| status | status . success ()) { ProgramKind :: Ssh } else { ProgramKind :: Simple } ; } Ok (kind) }
};
}
