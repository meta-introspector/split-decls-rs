// Generated macro for clone_command (function)
macro_rules! Depcrate_common_compile_cclone_command {
() => {
// Module: crate::common::compile_c
// Provides: {"clone_command"}
// Dependencies: {}
fn clone_command (command : & std :: process :: Command) -> std :: process :: Command { let mut cmd = std :: process :: Command :: new (command . get_program ()) ; if let Some (current_dir) = command . get_current_dir () { cmd . current_dir (current_dir) ; } cmd . args (command . get_args ()) ; for (key , val) in command . get_envs () { cmd . env (key , val . unwrap_or_default ()) ; } cmd }
};
}
