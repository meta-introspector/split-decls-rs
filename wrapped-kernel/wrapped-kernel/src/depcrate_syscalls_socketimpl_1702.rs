// Generated macro for impl_1702 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1702 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1702"}
// Dependencies: {}
impl Sock { pub fn from_bits (bits : i32) -> Option < (Self , SockFlags) > { let sock = Sock :: try_from (bits as u8) . ok () ? ; let flags = SockFlags :: from_bits_retain (bits & ! 0xff) ; Some ((sock , flags)) } }
};
}
