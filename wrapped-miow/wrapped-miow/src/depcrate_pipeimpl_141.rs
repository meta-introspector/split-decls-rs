// Generated macro for impl_141 (impl)
macro_rules! Depcrate_pipeimpl_141 {
() => {
// Module: crate::pipe
// Provides: {"impl_141"}
// Dependencies: {}
impl FromRawHandle for NamedPipe { unsafe fn from_raw_handle (handle : RawHandle) -> NamedPipe { NamedPipe (Handle :: new (handle as HANDLE)) } }
};
}
