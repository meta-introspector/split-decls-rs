// Generated macro for impl_73 (impl)
macro_rules! Depcrate_os_iocp_afdimpl_73 {
() => {
// Module: crate::os::iocp::afd
// Provides: {"impl_73"}
// Dependencies: {}
impl < T > fmt :: Debug for Afd < T > { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { struct WriteAsHex (HANDLE) ; impl fmt :: Debug for WriteAsHex { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{:010x}" , self . 0 as usize) } } f . debug_struct ("Afd") . field ("handle" , & WriteAsHex (self . handle)) . finish () } }
};
}
