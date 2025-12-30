// Generated macro for impl_146 (impl)
macro_rules! Depcrate_os_windowsimpl_146 {
() => {
// Module: crate::os::windows
// Provides: {"impl_146"}
// Dependencies: {}
impl < T > core :: ops :: Deref for Symbol < T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * ((& self . pointer) as * const FARPROC as * const T) } } }
};
}
