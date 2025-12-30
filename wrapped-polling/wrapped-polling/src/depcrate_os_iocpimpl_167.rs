// Generated macro for impl_167 (impl)
macro_rules! Depcrate_os_iocpimpl_167 {
() => {
// Module: crate::os::iocp
// Provides: {"impl_167"}
// Dependencies: {}
impl Drop for WaitHandle { fn drop (& mut self) { unsafe { UnregisterWait (self . 0 as _) ; } } }
};
}
