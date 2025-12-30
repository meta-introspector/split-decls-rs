// Generated macro for impl_1704 (impl)
macro_rules! Depcrate_syscalls_socketimpl_1704 {
() => {
// Module: crate::syscalls::socket
// Provides: {"impl_1704"}
// Dependencies: {}
impl From < Ipv4Addr > for in_addr { fn from (value : Ipv4Addr) -> Self { Self { s_addr : u32 :: from_ne_bytes (value . octets ()) , } } }
};
}
