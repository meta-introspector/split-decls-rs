// Generated macro for env_var_os (function)
macro_rules! Depcrate_envenv_var_os {
() => {
// Module: crate::env
// Provides: {"env_var_os"}
// Dependencies: {}
# [track_caller] # [must_use] pub fn env_var_os (name : & str) -> OsString { match std :: env :: var_os (name) { Some (v) => v , None => panic ! ("failed to retrieve environment variable {name:?}") , } }
};
}
