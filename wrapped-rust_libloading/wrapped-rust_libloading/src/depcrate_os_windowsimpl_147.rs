// Generated macro for impl_147 (impl)
macro_rules! Depcrate_os_windowsimpl_147 {
() => {
// Module: crate::os::windows
// Provides: {"impl_147"}
// Dependencies: {}
impl < T > fmt :: Debug for Symbol < T > { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { match self . pointer { None => f . write_str ("Symbol@0x0") , Some (ptr) => f . write_fmt (format_args ! ("Symbol@{:p}" , ptr as * const ())) , } } }
};
}
