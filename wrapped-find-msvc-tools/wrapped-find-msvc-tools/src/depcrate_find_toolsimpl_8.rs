// Generated macro for impl_8 (impl)
macro_rules! Depcrate_find_toolsimpl_8 {
() => {
// Module: crate::find_tools
// Provides: {"impl_8"}
// Dependencies: {}
impl Deref for Env { type Target = OsStr ; fn deref (& self) -> & Self :: Target { match self { Env :: Owned (os_str) => os_str , Env :: Arced (os_str) => os_str , } } }
};
}
