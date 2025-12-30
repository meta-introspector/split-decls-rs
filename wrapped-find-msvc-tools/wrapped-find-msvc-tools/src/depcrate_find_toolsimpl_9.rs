// Generated macro for impl_9 (impl)
macro_rules! Depcrate_find_toolsimpl_9 {
() => {
// Module: crate::find_tools
// Provides: {"impl_9"}
// Dependencies: {}
impl From < Env > for PathBuf { fn from (env : Env) -> Self { match env { Env :: Owned (os_str) => PathBuf :: from (os_str) , Env :: Arced (os_str) => PathBuf :: from (os_str . deref ()) , } } }
};
}
