// Generated macro for impl_108 (impl)
macro_rules! Depcrate_os_iocp_portimpl_108 {
() => {
// Module: crate::os::iocp::port
// Provides: {"impl_108"}
// Dependencies: {}
unsafe impl < T : Completion > CompletionHandle for Pin < & T > { type Completion = T ; fn get (& self) -> Pin < & Self :: Completion > { * self } fn into_ptr (this : Self) -> * mut OVERLAPPED { unsafe { Pin :: into_inner_unchecked (this) as * const T as * mut OVERLAPPED } } unsafe fn from_ptr (ptr : * mut OVERLAPPED) -> Self { Pin :: new_unchecked (& * (ptr as * const T)) } fn as_ptr (& self) -> * mut OVERLAPPED { self . get_ref () as * const T as * mut OVERLAPPED } }
};
}
