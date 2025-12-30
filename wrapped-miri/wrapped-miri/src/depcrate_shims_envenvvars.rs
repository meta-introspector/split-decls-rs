// Generated macro for EnvVars (enum)
macro_rules! Depcrate_shims_envEnvVars {
() => {
// Module: crate::shims::env
// Provides: {"EnvVars"}
// Dependencies: {}
# [derive (Default)] pub enum EnvVars < 'tcx > { # [default] Uninit , Unix (UnixEnvVars < 'tcx >) , Windows (WindowsEnvVars) , }
};
}
