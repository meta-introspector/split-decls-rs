// Generated macro for impl_176 (impl)
macro_rules! Depcrate_os_iocpimpl_176 {
() => {
// Module: crate::os::iocp
// Provides: {"impl_176"}
// Dependencies: {}
impl fmt :: Display for AfdError { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { write ! (f , "{}: {}\nThis error is usually caused by running on old Windows or Wine" , self . description , & self . system) } }
};
}
