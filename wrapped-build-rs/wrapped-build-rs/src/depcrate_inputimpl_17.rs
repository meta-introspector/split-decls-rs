// Generated macro for impl_17 (impl)
macro_rules! Depcrate_inputimpl_17 {
() => {
// Module: crate::input
// Provides: {"impl_17"}
// Dependencies: {}
impl Env for ProcessEnv { fn get (& self , key : & str) -> Option < std :: ffi :: OsString > { std :: env :: var_os (key) } fn is_present (& self , key : & str) -> bool { self . get (key) . is_some () } }
};
}
