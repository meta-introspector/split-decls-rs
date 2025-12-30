// Generated macro for vars (function)
macro_rules! Depcrate_envvars {
() => {
// Module: crate::env
// Provides: {"vars"}
// Dependencies: {}
pub fn vars () -> Iter < 'static , String , String > { CLI . get () . unwrap () . env_vars . iter () }
};
}
