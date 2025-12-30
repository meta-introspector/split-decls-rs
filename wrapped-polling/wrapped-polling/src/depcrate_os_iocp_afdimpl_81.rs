// Generated macro for impl_81 (impl)
macro_rules! Depcrate_os_iocp_afdimpl_81 {
() => {
// Module: crate::os::iocp::afd
// Provides: {"impl_81"}
// Dependencies: {}
impl < T > From < T > for IoStatusBlock < T > { fn from (data : T) -> Self { Self { iosb : UnsafeCell :: new (unsafe { std :: mem :: zeroed () }) , in_use : AtomicBool :: new (false) , data , _marker : PhantomPinned , } } }
};
}
