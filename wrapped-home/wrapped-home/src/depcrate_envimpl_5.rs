// Generated macro for impl_5 (impl)
macro_rules! Depcrate_envimpl_5 {
() => {
// Module: crate::env
// Provides: {"impl_5"}
// Dependencies: {}
impl Env for OsEnv { fn home_dir (& self) -> Option < PathBuf > { crate :: home_dir_inner () } fn current_dir (& self) -> io :: Result < PathBuf > { std :: env :: current_dir () } fn var_os (& self , key : & str) -> Option < OsString > { std :: env :: var_os (key) } }
};
}
