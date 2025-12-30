// Generated macro for impl_32 (impl)
macro_rules! Depcrate_windowsimpl_32 {
() => {
// Module: crate::windows
// Provides: {"impl_32"}
// Dependencies: {}
impl CommandExt for Command { fn creation_flags (& mut self , flags : u32) -> & mut Command { self . inner . creation_flags (flags) ; self } fn raw_arg < S : AsRef < OsStr > > (& mut self , text_to_append_as_is : S) -> & mut Command { self . inner . raw_arg (text_to_append_as_is) ; self } }
};
}
