// Generated macro for impl_157 (impl)
macro_rules! Depcrate_os_iocpimpl_157 {
() => {
// Module: crate::os::iocp
// Provides: {"impl_157"}
// Dependencies: {}
impl fmt :: Debug for PacketInner { fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: Wakeup { .. } => f . write_str ("Wakeup { .. }") , Self :: Custom { event } => f . debug_struct ("Custom") . field ("event" , event) . finish () , Self :: Socket { socket , .. } => f . debug_struct ("Socket") . field ("packet" , & "..") . field ("socket" , socket) . finish () , Self :: Waitable { handle } => { f . debug_struct ("Waitable") . field ("handle" , handle) . finish () } } } }
};
}
