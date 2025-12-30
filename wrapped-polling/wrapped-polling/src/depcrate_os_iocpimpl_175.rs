// Generated macro for impl_175 (impl)
macro_rules! Depcrate_os_iocpimpl_175 {
() => {
// Module: crate::os::iocp
// Provides: {"impl_175"}
// Dependencies: {}
impl fmt :: Debug for AfdError { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { f . debug_struct ("AfdError") . field ("description" , & self . description) . field ("system" , & self . system) . field ("note" , & "probably caused by old Windows or Wine") . finish () } }
};
}
