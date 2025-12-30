// Generated macro for impl_7 (impl)
macro_rules! Depcrate_commandimpl_7 {
() => {
// Module: crate::command
// Provides: {"impl_7"}
// Dependencies: {}
impl Command { # [doc = " Produce the name of the command as known by the server side."] pub fn as_str (& self) -> & 'static str { match self { Command :: LsRefs => "ls-refs" , Command :: Fetch => "fetch" , } } }
};
}
