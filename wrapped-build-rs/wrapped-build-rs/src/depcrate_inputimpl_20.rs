// Generated macro for impl_20 (impl)
macro_rules! Depcrate_inputimpl_20 {
() => {
// Module: crate::input
// Provides: {"impl_20"}
// Dependencies: {}
impl < E : Env > Env for RerunIfEnvChanged < E > { # [track_caller] fn get (& self , key : & str) -> Option < std :: ffi :: OsString > { rerun_if_env_changed (key) ; self . 0 . get (key) } # [track_caller] fn is_present (& self , key : & str) -> bool { self . get (key) . is_some () } }
};
}
