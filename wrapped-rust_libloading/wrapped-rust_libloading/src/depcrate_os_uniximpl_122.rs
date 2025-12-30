// Generated macro for impl_122 (impl)
macro_rules! Depcrate_os_uniximpl_122 {
() => {
// Module: crate::os::unix
// Provides: {"impl_122"}
// Dependencies: {}
impl < T > core :: ops :: Deref for Symbol < T > { type Target = T ; fn deref (& self) -> & T { unsafe { & * (& self . pointer as * const * mut _ as * const T) } } }
};
}
