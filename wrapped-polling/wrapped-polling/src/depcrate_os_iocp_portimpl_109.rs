// Generated macro for impl_109 (impl)
macro_rules! Depcrate_os_iocp_portimpl_109 {
() => {
// Module: crate::os::iocp::port
// Provides: {"impl_109"}
// Dependencies: {}
unsafe impl < T : Completion > CompletionHandle for Pin < Arc < T > > { type Completion = T ; fn get (& self) -> Pin < & Self :: Completion > { self . as_ref () } fn into_ptr (this : Self) -> * mut OVERLAPPED { unsafe { Arc :: into_raw (Pin :: into_inner_unchecked (this)) as * const T as * mut OVERLAPPED } } unsafe fn from_ptr (ptr : * mut OVERLAPPED) -> Self { Pin :: new_unchecked (Arc :: from_raw (ptr as * const T)) } fn as_ptr (& self) -> * mut OVERLAPPED { self . as_ref () . get_ref () as * const T as * mut OVERLAPPED } }
};
}
