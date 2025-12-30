// Generated macro for impl_226 (impl)
macro_rules! Depcrate_multiimpl_226 {
() => {
// Module: crate::multi
// Provides: {"impl_226"}
// Dependencies: {}
impl fmt :: Debug for WaitFd { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { f . debug_struct ("WaitFd") . field ("fd" , & self . inner . fd) . field ("events" , & self . inner . fd) . field ("revents" , & self . inner . fd) . finish () } }
};
}
