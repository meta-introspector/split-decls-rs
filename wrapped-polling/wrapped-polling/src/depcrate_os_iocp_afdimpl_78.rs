// Generated macro for impl_78 (impl)
macro_rules! Depcrate_os_iocp_afdimpl_78 {
() => {
// Module: crate::os::iocp::afd
// Provides: {"impl_78"}
// Dependencies: {}
impl < T : fmt :: Debug > fmt :: Debug for IoStatusBlock < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("IoStatusBlock") . field ("iosb" , & "..") . field ("in_use" , & self . in_use) . field ("data" , & self . data) . finish () } }
};
}
