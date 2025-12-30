// Generated macro for impl_60 (impl)
macro_rules! Depcrate_processimpl_60 {
() => {
// Module: crate::process
// Provides: {"impl_60"}
// Dependencies: {}
impl Drop for Process { fn drop (& mut self) { unsafe { ClosePseudoConsole (self . _console) ; let _ = CloseHandle (self . _proc . hProcess) ; let _ = CloseHandle (self . _proc . hThread) ; DeleteProcThreadAttributeList (self . _proc_info . lpAttributeList) ; let _ : Box < u8 > = Box :: from_raw (self . _proc_info . lpAttributeList . 0 as * mut u8) ; let _ = CloseHandle (self . input) ; let _ = CloseHandle (self . output) ; } } }
};
}
