// Generated macro for impl_52 (impl)
macro_rules! Depcrate_iocpimpl_52 {
() => {
// Module: crate::iocp
// Provides: {"impl_52"}
// Dependencies: {}
impl FromRawHandle for CompletionPort { unsafe fn from_raw_handle (handle : RawHandle) -> CompletionPort { CompletionPort { handle : Handle :: new (handle as HANDLE) , } } }
};
}
