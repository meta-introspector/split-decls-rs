// Generated macro for var (function)
macro_rules! Depcrate_envvar {
() => {
// Module: crate::env
// Provides: {"var"}
// Dependencies: {}
# [allow (dead_code)] pub fn var (key : & str) -> Option < & String > { CLI . get () . unwrap () . env_vars . get (key) }
};
}
