// Generated macro for env_var (function)
macro_rules! Depcrate_envenv_var {
() => {
// Module: crate::env
// Provides: {"env_var"}
// Dependencies: {}
# [track_caller] # [must_use] pub fn env_var (name : & str) -> String { match std :: env :: var (name) { Ok (v) => v , Err (err) => panic ! ("failed to retrieve environment variable {name:?}: {err:?}") , } }
};
}
