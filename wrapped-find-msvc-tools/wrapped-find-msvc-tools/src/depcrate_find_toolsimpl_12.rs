// Generated macro for impl_12 (impl)
macro_rules! Depcrate_find_toolsimpl_12 {
() => {
// Module: crate::find_tools
// Provides: {"impl_12"}
// Dependencies: {}
impl EnvGetter for StdEnvGetter { # [allow (clippy :: disallowed_methods)] fn get_env (& self , name : & 'static str) -> Option < Env > { env :: var_os (name) . map (Env :: Owned) } }
};
}
