// Generated macro for impl_113 (impl)
macro_rules! Depcrate_os_iocp_portimpl_113 {
() => {
// Module: crate::os::iocp::port
// Provides: {"impl_113"}
// Dependencies: {}
impl < T > fmt :: Debug for IoCompletionPort < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct WriteAsHex (HANDLE) ; impl fmt :: Debug for WriteAsHex { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:010x}" , self . 0 as usize) } } f . debug_struct ("IoCompletionPort") . field ("handle" , & WriteAsHex (self . handle)) . finish () } }
};
}
